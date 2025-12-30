// Generated macro for impl_183 (impl)
macro_rules! Depcrate_opsimpl_183 {
() => {
// Module: crate::ops
// Provides: {"impl_183"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'this , T > BatchInvert < & 'this mut [Self] > for T where T : Field , { type Output = CtOption < & 'this mut [Self] > ; fn batch_invert (field_elements : & 'this mut [Self]) -> CtOption < & 'this mut [Self] > { let mut field_elements_pad : Vec < Self > = vec ! [Self :: default () ; field_elements . len ()] ; let inversion_succeeded = invert_batch_internal (field_elements , & mut field_elements_pad , invert) ; CtOption :: new (field_elements , inversion_succeeded) } }
};
}
