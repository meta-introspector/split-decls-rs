macro_rules! deps {
    () => {
        Directory!();
    };
}

macro_rules! decode_directory_block {
    () => {
        deps!();
        fn decode_directory_block < 'a > (data : & 'a [u8] , directories : & mut Vec < Directory >) -> Option < & 'a [u8] > { let (num_untracked , data) = var_int (data) ? ; let (num_dirs , data) = var_int (data) ? ; let (name , mut data) = split_at_byte_exclusive (data , 0) ? ; let mut untracked_entries = Vec :: < BString > :: with_capacity (num_untracked . try_into () . ok () ?) ; for _ in 0 .. num_untracked { let (name , rest) = split_at_byte_exclusive (data , 0) ? ; data = rest ; untracked_entries . push (name . into ()) ; } let index = directories . len () ; directories . push (Directory { name : name . into () , untracked_entries , sub_directories : Vec :: with_capacity (num_dirs . try_into () . ok () ?) , stat : None , exclude_file_oid : None , check_only : false , }) ; for _ in 0 .. num_dirs { let subdir_index = directories . len () ; let rest = decode_directory_block (data , directories) ? ; data = rest ; directories [index] . sub_directories . push (subdir_index) ; } data . into () }
    };
}

decode_directory_block!();