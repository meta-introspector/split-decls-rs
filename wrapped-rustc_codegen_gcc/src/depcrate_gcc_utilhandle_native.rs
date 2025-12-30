// Generated macro for handle_native (function)
macro_rules! Depcrate_gcc_utilhandle_native {
() => {
// Module: crate::gcc_util
// Provides: {"handle_native"}
// Dependencies: {}
fn handle_native (name : & str) -> & str { if name != "native" { return arch_to_gcc (name) ; } # [cfg (feature = "master")] { let context = Context :: default () ; context . get_target_info () . arch () . unwrap () . to_str () . unwrap () } # [cfg (not (feature = "master"))] unimplemented ! () ; }
};
}
