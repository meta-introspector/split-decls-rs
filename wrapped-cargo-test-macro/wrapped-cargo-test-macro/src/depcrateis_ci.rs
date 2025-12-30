// Generated macro for is_ci (function)
macro_rules! Depcrateis_ci {
() => {
// Module: crate
// Provides: {"is_ci"}
// Dependencies: {}
# [doc = " Whether or not this running in a Continuous Integration environment."] fn is_ci () -> bool { option_env ! ("CI") . is_some () || option_env ! ("TF_BUILD") . is_some () }
};
}
