// Generated macro for impl_182 (impl)
macro_rules! Depcrate_opsimpl_182 {
() => {
// Module: crate::ops
// Provides: {"impl_182"}
// Dependencies: {}
impl < const N : usize , T > BatchInvert < [T ; N] > for T where T : Field , { type Output = CtOption < [Self ; N] > ; fn batch_invert (mut field_elements : [Self ; N]) -> CtOption < [Self ; N] > { let mut field_elements_pad = [Self :: default () ; N] ; let inversion_succeeded = invert_batch_internal (& mut field_elements , & mut field_elements_pad , invert) ; CtOption :: new (field_elements , inversion_succeeded) } }
};
}
