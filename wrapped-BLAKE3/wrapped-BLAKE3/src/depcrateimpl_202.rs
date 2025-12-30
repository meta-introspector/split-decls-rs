// Generated macro for impl_202 (impl)
macro_rules! Depcrateimpl_202 {
() => {
// Module: crate
// Provides: {"impl_202"}
// Dependencies: {}
# [doc = " This implementation is constant-time if the target is 32 bytes long."] impl PartialEq < [u8] > for Hash { # [inline] fn eq (& self , other : & [u8]) -> bool { constant_time_eq :: constant_time_eq (& self . 0 , other) } }
};
}
