// Generated macro for impl_297 (impl)
macro_rules! Depcrate_valueimpl_297 {
() => {
// Module: crate::value
// Provides: {"impl_297"}
// Dependencies: {}
# [doc = " Special case to allow `Value::from(b\"byte string\")`"] impl < const N : usize > From < & 'static [u8 ; N] > for Value { fn from (value : & 'static [u8 ; N]) -> Self { Self :: Bytes (Vec :: from (* value)) } }
};
}
