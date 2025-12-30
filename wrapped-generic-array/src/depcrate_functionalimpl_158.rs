// Generated macro for impl_158 (impl)
macro_rules! Depcrate_functionalimpl_158 {
() => {
// Module: crate::functional
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'a , T , U , S : MappedGenericSequence < T , U > > MappedGenericSequence < T , U > for & 'a S where & 'a S : GenericSequence < T > , S : GenericSequence < T , Length = < & 'a S as GenericSequence < T > > :: Length > , { type Mapped = < S as MappedGenericSequence < T , U > > :: Mapped ; }
};
}
