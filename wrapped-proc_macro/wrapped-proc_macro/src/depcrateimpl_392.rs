// Generated macro for impl_392 (impl)
macro_rules! Depcrateimpl_392 {
() => {
// Module: crate
// Provides: {"impl_392"}
// Dependencies: {}
# [doc = " Prints the literal as a string that should be losslessly convertible"] # [doc = " back into the same literal (except for possible rounding for floating point literals)."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Display for Literal { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . with_stringify_parts (| parts | { for part in parts { fmt :: Display :: fmt (part , f) ? ; } Ok (()) }) } }
};
}
