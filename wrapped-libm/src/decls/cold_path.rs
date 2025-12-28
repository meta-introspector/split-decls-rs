macro_rules! cold_path {
    () => {
        # [doc = " Hint to the compiler that the current path is cold."] pub fn cold_path () { # [cfg (intrinsics_enabled)] core :: intrinsics :: cold_path () ; }
    };
}

cold_path!();