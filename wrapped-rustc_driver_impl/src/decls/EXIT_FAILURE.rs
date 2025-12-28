macro_rules! EXIT_FAILURE {
    () => {
        # [doc = " Exit status code used for compilation failures and invalid flags."] pub const EXIT_FAILURE : i32 = 1 ;
    };
}

EXIT_FAILURE!()