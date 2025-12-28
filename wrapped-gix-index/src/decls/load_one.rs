macro_rules! deps {
    () => {
        Mode!();
        Time!();
        Entry!();
        Stat!();
    };
}

macro_rules! load_one {
    () => {
        deps!();
        # [doc = " Note that `prev_path` is only useful if the version is V4"] fn load_one < 'a > (data : & 'a [u8] , path_backing : & mut Vec < u8 > , hash_len : usize , has_delta_paths : bool , prev_path_and_buf : Option < (Range < usize > , & mut Vec < u8 >) > ,) -> Option < (Entry , & 'a [u8]) > { let first_byte_of_entry = data . as_ptr () as usize ; let (ctime_secs , data) = read_u32 (data) ? ; let (ctime_nsecs , data) = read_u32 (data) ? ; let (mtime_secs , data) = read_u32 (data) ? ; let (mtime_nsecs , data) = read_u32 (data) ? ; let (dev , data) = read_u32 (data) ? ; let (ino , data) = read_u32 (data) ? ; let (mode , data) = read_u32 (data) ? ; let (uid , data) = read_u32 (data) ? ; let (gid , data) = read_u32 (data) ? ; let (size , data) = read_u32 (data) ? ; let (hash , data) = data . split_at_checked (hash_len) ? ; let (flags , data) = read_u16 (data) ? ; let flags = entry :: at_rest :: Flags :: from_bits_retain (flags) ; let (flags , data) = if flags . contains (entry :: at_rest :: Flags :: EXTENDED) { let (extended_flags , data) = read_u16 (data) ? ; let extended_flags = entry :: at_rest :: FlagsExtended :: from_bits (extended_flags) ? ; let extended_flags = extended_flags . to_flags () ? ; (flags . to_memory () | extended_flags , data) } else { (flags . to_memory () , data) } ; let start = path_backing . len () ; let data = if has_delta_paths { let (strip_len , data) = var_int (data) ? ; if let Some ((prev_path , buf)) = prev_path_and_buf { let end = prev_path . end . checked_sub (strip_len . try_into () . ok () ?) ? ; let copy_len = end . checked_sub (prev_path . start) ? ; if copy_len > 0 { buf . resize (copy_len , 0) ; buf . copy_from_slice (& path_backing [prev_path . start .. end]) ; path_backing . extend_from_slice (buf) ; } } let (path , data) = split_at_byte_exclusive (data , 0) ? ; path_backing . extend_from_slice (path) ; data } else { let (path , data) = if flags . contains (entry :: Flags :: PATH_LEN) { split_at_byte_exclusive (data , 0) ? } else { let path_len = (flags . bits () & entry :: Flags :: PATH_LEN . bits ()) as usize ; let (path , data) = data . split_at_checked (path_len) ? ; (path , skip_padding (data , first_byte_of_entry)) } ; path_backing . extend_from_slice (path) ; data } ; let path_range = start .. path_backing . len () ; Some ((Entry { stat : entry :: Stat { ctime : entry :: stat :: Time { secs : ctime_secs , nsecs : ctime_nsecs , } , mtime : entry :: stat :: Time { secs : mtime_secs , nsecs : mtime_nsecs , } , dev , ino , uid , gid , size , } , id : gix_hash :: ObjectId :: from_bytes_or_panic (hash) , flags : flags & ! entry :: Flags :: PATH_LEN , mode : entry :: Mode :: from_bits_truncate (mode) , path : path_range , } , data ,)) }
    };
}

load_one!();