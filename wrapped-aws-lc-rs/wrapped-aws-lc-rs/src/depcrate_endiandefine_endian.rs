// Generated macro for define_endian (macro)
macro_rules! Depcrate_endiandefine_endian {
() => {
// Module: crate::endian
// Provides: {"define_endian"}
// Dependencies: {}
macro_rules ! define_endian { ($ endian : ident) => { # [derive (Copy , Clone)] # [repr (transparent)] pub struct $ endian < T > (T) ; } ; }
};
}
