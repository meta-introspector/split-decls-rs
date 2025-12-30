// Generated macro for impl_102 (impl)
macro_rules! Depcrate_devimpl_102 {
() => {
// Module: crate::dev
// Provides: {"impl_102"}
// Dependencies: {}
impl ConstantTimeEq for AffinePoint { fn ct_eq (& self , other : & Self) -> Choice { match (self , other) { (Self :: FixedBaseOutput (scalar) , Self :: FixedBaseOutput (other_scalar)) => { scalar . ct_eq (other_scalar) } (Self :: Identity , Self :: Identity) | (Self :: Generator , Self :: Generator) => 1 . into () , (Self :: Other (point) , Self :: Other (other_point)) => u8 :: from (point == other_point) . into () , _ => 0 . into () , } } }
};
}
