// Generated macro for impl_184 (impl)
macro_rules! Depcrate_opsimpl_184 {
() => {
// Module: crate::ops
// Provides: {"impl_184"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > BatchInvert < & [Self] > for T where T : Field , { type Output = CtOption < Vec < Self > > ; fn batch_invert (field_elements : & [Self]) -> CtOption < Vec < Self > > { let mut field_elements : Vec < Self > = field_elements . to_owned () ; let mut field_elements_pad : Vec < Self > = vec ! [Self :: default () ; field_elements . len ()] ; let inversion_succeeded = invert_batch_internal (& mut field_elements , & mut field_elements_pad , invert) ; CtOption :: new (field_elements , inversion_succeeded) } }
};
}
