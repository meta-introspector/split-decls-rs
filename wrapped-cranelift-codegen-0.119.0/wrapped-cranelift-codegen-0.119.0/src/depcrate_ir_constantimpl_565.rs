// Generated macro for impl_565 (impl)
macro_rules! Depcrate_ir_constantimpl_565 {
() => {
// Module: crate::ir::constant
// Provides: {"impl_565"}
// Dependencies: {}
impl TryFrom < & ConstantData > for Ieee128 { type Error = < [u8 ; 16] as TryFrom < & 'static [u8] > > :: Error ; fn try_from (value : & ConstantData) -> Result < Self , Self :: Error > { Ok (Ieee128 :: with_bits (u128 :: from_le_bytes (value . as_slice () . try_into () ? ,))) } }
};
}
