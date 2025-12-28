macro_rules! remove_if_exists {
    () => {
        fn remove_if_exists (name : & str) { if let Err (e) = Command :: new ("docker") . args (& ["container" , "rm" , "--force" , name]) . output () { panic ! ("failed to run docker: {e}") ; } }
    };
}

remove_if_exists!();