// Generated macro for impl_566 (impl)
macro_rules! Depcrate_ir_constantimpl_566 {
() => {
// Module: crate::ir::constant
// Provides: {"impl_566"}
// Dependencies: {}
impl ConstantData { # [doc = " Return the number of bytes in the constant."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Check if the constant contains any bytes."] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Return the data as a slice."] pub fn as_slice (& self) -> & [u8] { self . 0 . as_slice () } # [doc = " Convert the data to a vector."] pub fn into_vec (self) -> Vec < u8 > { self . 0 } # [doc = " Iterate over the constant's bytes."] pub fn iter (& self) -> Iter < u8 > { self . 0 . iter () } # [doc = " Add new bytes to the constant data."] pub fn append (mut self , bytes : impl IntoBytes) -> Self { let mut to_add = bytes . into_bytes () ; self . 0 . append (& mut to_add) ; self } # [doc = " Expand the size of the constant data to `expected_size` number of bytes by adding zeroes"] # [doc = " in the high-order byte slots."] pub fn expand_to (mut self , expected_size : usize) -> Self { if self . len () > expected_size { panic ! ("The constant data is already expanded beyond {expected_size} bytes") } self . 0 . resize (expected_size , 0) ; self } }
};
}
