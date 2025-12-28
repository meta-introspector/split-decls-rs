macro_rules! convert_path {
    () => {
        fn convert_path (path : & OsStr) -> io :: Result < Vec < u16 > > { let mut maybe_result : Vec < u16 > = path . encode_wide () . collect () ; if maybe_result . contains (& 0) { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "path contains NUL byte(s)" ,)) ; } maybe_result . push (0) ; Ok (maybe_result) }
    };
}

convert_path!()