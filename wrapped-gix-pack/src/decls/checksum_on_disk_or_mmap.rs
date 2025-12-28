macro_rules! deps {
    () => {
        Error!();
        Kind!();
    };
}

macro_rules! checksum_on_disk_or_mmap {
    () => {
        deps!();
        # [doc = " Calculate the hash of the given kind by trying to read the file from disk at `data_path` or falling back on the mapped content in `data`."] # [doc = " `Ok(expected)` or [`checksum::Error::Verify`] is returned if the hash matches or mismatches."] # [doc = " If the [`checksum::Error::Interrupted`] is returned, the operation was interrupted."] pub fn checksum_on_disk_or_mmap (data_path : & Path , data : & [u8] , expected : gix_hash :: ObjectId , object_hash : gix_hash :: Kind , progress : & mut dyn Progress , should_interrupt : & AtomicBool ,) -> Result < gix_hash :: ObjectId , checksum :: Error > { let data_len_without_trailer = data . len () - object_hash . len_in_bytes () ; let actual = match gix_hash :: bytes_of_file (data_path , data_len_without_trailer as u64 , object_hash , progress , should_interrupt ,) { Ok (id) => id , Err (gix_hash :: io :: Error :: Io (err)) if err . kind () == std :: io :: ErrorKind :: Interrupted => { return Err (checksum :: Error :: Interrupted) ; } Err (gix_hash :: io :: Error :: Io (_io_err)) => { let start = std :: time :: Instant :: now () ; let mut hasher = gix_hash :: hasher (object_hash) ; hasher . update (& data [.. data_len_without_trailer]) ; progress . inc_by (data_len_without_trailer) ; progress . show_throughput (start) ; hasher . try_finalize () ? } Err (gix_hash :: io :: Error :: Hasher (err)) => return Err (checksum :: Error :: Hasher (err)) , } ; actual . verify (& expected) ? ; Ok (actual) }
    };
}

checksum_on_disk_or_mmap!()