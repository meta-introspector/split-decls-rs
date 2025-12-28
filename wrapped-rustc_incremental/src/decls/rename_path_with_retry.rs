macro_rules! deps {
    () => {
        Ok!();
    };
}

macro_rules! rename_path_with_retry {
    () => {
        deps!();
        fn rename_path_with_retry (from : & Path , to : & Path , mut retries_left : usize) -> std :: io :: Result < () > { loop { match std_fs :: rename (from , to) { Ok (()) => return Ok (()) , Err (e) => { if retries_left > 0 && e . kind () == ErrorKind :: PermissionDenied { std :: thread :: sleep (Duration :: from_millis (50)) ; retries_left -= 1 ; } else { return Err (e) ; } } } } }
    };
}

rename_path_with_retry!()