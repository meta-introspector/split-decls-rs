// Generated macro for macro_5 (macro)
macro_rules! Depcratemacro_5 {
() => {
// Module: crate
// Provides: {"macro_5"}
// Dependencies: {}
# [cfg (any (all (feature = "compiler-rt" , feature = "gnustep-1-7") , all (feature = "gnustep-1-7" , feature = "unstable-objfw") , all (feature = "compiler-rt" , feature = "unstable-objfw") ,))] compile_error ! ("Only one runtime may be selected") ;
};
}
