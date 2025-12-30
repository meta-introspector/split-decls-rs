// Generated macro for impl_185 (impl)
macro_rules! Depcrate_opsimpl_185 {
() => {
// Module: crate::ops
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > BatchInvert < Vec < Self > > for T where T : Field , { type Output = CtOption < Vec < Self > > ; fn batch_invert (mut field_elements : Vec < Self >) -> CtOption < Vec < Self > > { let mut field_elements_pad : Vec < Self > = vec ! [Self :: default () ; field_elements . len ()] ; let inversion_succeeded = invert_batch_internal (& mut field_elements , & mut field_elements_pad , invert) ; CtOption :: new (field_elements , inversion_succeeded) } }
};
}
