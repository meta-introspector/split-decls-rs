// Generated macro for impl_113 (impl)
macro_rules! Depcrate_devimpl_113 {
() => {
// Module: crate::dev
// Provides: {"impl_113"}
// Dependencies: {}
impl ConstantTimeEq for ProjectivePoint { fn ct_eq (& self , other : & Self) -> Choice { match (self , other) { (Self :: FixedBaseOutput (scalar) , Self :: FixedBaseOutput (other_scalar)) => { scalar . ct_eq (other_scalar) } (Self :: Identity , Self :: Identity) | (Self :: Generator , Self :: Generator) => 1 . into () , (Self :: Other (point) , Self :: Other (other_point)) => point . ct_eq (other_point) , _ => 0 . into () , } } }
};
}
