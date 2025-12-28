macro_rules! other_85 {
    () => {
        # [doc = " Returns a version string such as \"1.46.0 (04488afe3 2020-08-24)\" when invoked by an in-tree tool."] pub macro version_str () { option_env ! ("CFG_VERSION") }
    };
}

other_85!();