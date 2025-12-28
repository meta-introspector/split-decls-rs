macro_rules! deps {
    () => {
        Path!();
        Error!();
    };
}

macro_rules! read_regular_file_content_with_size_limit {
    () => {
        deps!();
        fn read_regular_file_content_with_size_limit (path : & std :: path :: Path) -> std :: io :: Result < Vec < u8 > > { let mut file = std :: fs :: File :: open (path) ? ; let max_file_size = 1024 * 64 ; let file_size = file . metadata () ? . len () ; if file_size > max_file_size { return Err (std :: io :: Error :: other (format ! ("Refusing to open files larger than {} bytes, '{}' was {} bytes large" , max_file_size , path . display () , file_size))) ; } let mut buf = Vec :: with_capacity (512) ; file . read_to_end (& mut buf) ? ; Ok (buf) }
    };
}

read_regular_file_content_with_size_limit!()