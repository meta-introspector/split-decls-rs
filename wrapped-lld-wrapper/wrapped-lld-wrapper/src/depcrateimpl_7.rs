// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > UnwrapOrExitWith < T > for Option < T > { fn unwrap_or_exit_with (self , context : & str) -> T { self . unwrap_or_else (| | { eprintln ! ("lld-wrapper: {}" , context) ; process :: exit (1) ; }) } }
};
}
