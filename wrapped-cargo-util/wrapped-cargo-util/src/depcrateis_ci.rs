// Generated macro for is_ci (function)
macro_rules! Depcrateis_ci {
() => {
// Module: crate
// Provides: {"is_ci"}
// Dependencies: {}
# [doc = " Whether or not this running in a Continuous Integration environment."] pub fn is_ci () -> bool { std :: env :: var ("CI") . is_ok () || std :: env :: var ("TF_BUILD") . is_ok () }
};
}
