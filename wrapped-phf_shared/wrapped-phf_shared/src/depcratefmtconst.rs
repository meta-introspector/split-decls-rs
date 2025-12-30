// Generated macro for FmtConst (trait)
macro_rules! DepcrateFmtConst {
() => {
// Module: crate
// Provides: {"FmtConst"}
// Dependencies: {}
# [doc = " Trait for printing types with `const` constructors, used by `phf_codegen` and `phf_macros`."] pub trait FmtConst { # [doc = " Print a `const` expression representing this value."] fn fmt_const (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }
};
}
