// Generated macro for byte_order (function)
macro_rules! Depcrate_testsbyte_order {
() => {
// Module: crate::tests
// Provides: {"byte_order"}
// Dependencies: {}
# [test] fn byte_order () { # [cfg (target_endian = "little")] std :: eprintln ! ("LITTLE ENDIAN") ; # [cfg (target_endian = "big")] std :: eprintln ! ("BIG ENDIAN") ; }
};
}
