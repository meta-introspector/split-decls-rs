// Generated macro for impl_250 (impl)
macro_rules! Depcrateimpl_250 {
() => {
// Module: crate
// Provides: {"impl_250"}
// Dependencies: {}
impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for GenericArray < T , N > where GenericArray < U , N > : GenericSequence < U , Length = N > , { type Mapped = GenericArray < U , N > ; }
};
}
