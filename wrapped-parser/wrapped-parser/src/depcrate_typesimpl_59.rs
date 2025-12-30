// Generated macro for impl_59 (impl)
macro_rules! Depcrate_typesimpl_59 {
() => {
// Module: crate::types
// Provides: {"impl_59"}
// Dependencies: {}
impl Display for Type { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { self . base . fmt (f) ? ; if ! self . nullable { f . write_char ('!') ? ; } Ok (()) } }
};
}
