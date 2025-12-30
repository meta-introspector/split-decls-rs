// Generated macro for impl_14 (impl)
macro_rules! Depcrate_mapimpl_14 {
() => {
// Module: crate::map
// Provides: {"impl_14"}
// Dependencies: {}
impl < T , V > IntoIterator for ArenaMap < Idx < T > , V > { type Item = (Idx < T > , V) ; type IntoIter = ArenaMapIter < Idx < T > , V > ; fn into_iter (self) -> Self :: IntoIter { let iter = self . v . into_iter () . enumerate () ; Self :: IntoIter { iter , _ty : PhantomData } } }
};
}
