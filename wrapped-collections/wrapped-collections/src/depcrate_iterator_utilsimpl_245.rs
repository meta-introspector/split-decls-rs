// Generated macro for impl_245 (impl)
macro_rules! Depcrate_iterator_utilsimpl_245 {
() => {
// Module: crate::iterator_utils
// Provides: {"impl_245"}
// Dependencies: {}
impl < I , T : Eq > RangeListIteratorCoalescer < I , T > where I : Iterator < Item = CodePointMapRange < T > > , { pub fn new (iter : I) -> Self { Self { iter , peek : None } } }
};
}
