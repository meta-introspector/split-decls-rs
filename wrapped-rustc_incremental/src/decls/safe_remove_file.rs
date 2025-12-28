macro_rules! deps {
    () => {
        Ok!();
    };
}

macro_rules! safe_remove_file {
    () => {
        deps!();
        fn safe_remove_file (p : & Path) -> io :: Result < () > { match std_fs :: remove_file (p) { Err (err) if err . kind () == io :: ErrorKind :: NotFound => Ok (()) , result => result , } }
    };
}

safe_remove_file!()