// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T , E : Display > UnwrapOrExitWith < T > for Result < T , E > { fn unwrap_or_exit_with (self , context : & str) -> T { self . unwrap_or_else (| err | { eprintln ! ("lld-wrapper: {}: {}" , context , err) ; process :: exit (1) ; }) } }
};
}
