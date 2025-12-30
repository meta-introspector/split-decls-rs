// Generated macro for macro_1020 (macro)
macro_rules! Depcrate_fdmacro_1020 {
() => {
// Module: crate::fd
// Provides: {"macro_1020"}
// Dependencies: {}
bitflags ! { # [doc = " Options for checking file permissions or existence"] # [derive (Debug , Copy , Clone , Default , Eq , PartialEq)] pub struct AccessOption : i32 { # [doc = " Test for read permission"] const R_OK = 4 ; # [doc = " Test for write permission"] const W_OK = 2 ; # [doc = " Test for execution permission"] const X_OK = 1 ; # [doc = " Test for existence"] const F_OK = 0 ; } }
};
}
