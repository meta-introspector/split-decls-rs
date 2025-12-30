// Generated macro for impl_159 (impl)
macro_rules! Depcrate_functionalimpl_159 {
() => {
// Module: crate::functional
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a , T , U , S : MappedGenericSequence < T , U > > MappedGenericSequence < T , U > for & 'a mut S where & 'a mut S : GenericSequence < T > , S : GenericSequence < T , Length = < & 'a mut S as GenericSequence < T > > :: Length > , { type Mapped = < S as MappedGenericSequence < T , U > > :: Mapped ; }
};
}
