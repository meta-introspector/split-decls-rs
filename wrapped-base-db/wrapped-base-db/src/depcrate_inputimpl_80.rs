// Generated macro for impl_80 (impl)
macro_rules! Depcrate_inputimpl_80 {
() => {
// Module: crate::input
// Provides: {"impl_80"}
// Dependencies: {}
impl From < Env > for Vec < (String , String) > { fn from (env : Env) -> Vec < (String , String) > { let mut entries : Vec < _ > = env . entries . into_iter () . collect () ; entries . sort () ; entries } }
};
}
