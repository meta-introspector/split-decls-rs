macro_rules! check_zst {
    () => {
        fn check_zst < T > () -> Result < () > { if size_of :: < T > () == 0 { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_ZST_FORBIDDEN)) ; } Ok (()) }
    };
}

check_zst!();