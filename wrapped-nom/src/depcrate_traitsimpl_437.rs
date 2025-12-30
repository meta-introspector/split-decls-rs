// Generated macro for impl_437 (impl)
macro_rules! Depcrate_traitsimpl_437 {
() => {
// Module: crate::traits
// Provides: {"impl_437"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl ExtendInto for & [u8] { type Item = u8 ; type Extender = Vec < u8 > ; # [inline] fn new_builder (& self) -> Vec < u8 > { Vec :: new () } # [inline] fn extend_into (& self , acc : & mut Vec < u8 >) { acc . extend_from_slice (self) ; } }
};
}
