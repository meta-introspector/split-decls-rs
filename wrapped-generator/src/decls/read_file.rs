macro_rules! read_file {
    () => {
        fn read_file < P : AsRef < Path > > (path : P) -> io :: Result < String > { let mut file = File :: open (path . as_ref ()) ? ; let mut string = String :: new () ; file . read_to_string (& mut string) ? ; Ok (string) }
    };
}

read_file!();