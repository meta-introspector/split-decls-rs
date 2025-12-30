// Generated macro for impl_178 (impl)
macro_rules! Depcrate_serimpl_178 {
() => {
// Module: crate::ser
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , W : fmt :: Write > Compound < 'a , W > { fn new (ser : & 'a mut Serializer < W > , newtype_variant : bool) -> Self { Compound { ser , state : State :: First , newtype_variant , sequence_index : 0 , } } }
};
}
