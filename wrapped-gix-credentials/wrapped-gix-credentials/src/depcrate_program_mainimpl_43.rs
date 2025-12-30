// Generated macro for impl_43 (impl)
macro_rules! Depcrate_program_mainimpl_43 {
() => {
// Module: crate::program::main
// Provides: {"impl_43"}
// Dependencies: {}
impl Action { # [doc = " Return ourselves as string representation, similar to what would be passed as argument to a credential helper."] pub fn as_str (& self) -> & 'static str { match self { Action :: Get => "get" , Action :: Store => "store" , Action :: Erase => "erase" , } } }
};
}
