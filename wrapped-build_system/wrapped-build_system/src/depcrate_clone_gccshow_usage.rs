// Generated macro for show_usage (function)
macro_rules! Depcrate_clone_gccshow_usage {
() => {
// Module: crate::clone_gcc
// Provides: {"show_usage"}
// Dependencies: {}
fn show_usage () { println ! (r#"
`clone-gcc` command help:

    --out-path         : Location where the GCC repository will be cloned (default: `./gcc`)"#) ; ConfigInfo :: show_usage () ; println ! ("    --help                 : Show this help") ; }
};
}
