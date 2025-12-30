// Generated macro for impl_190 (impl)
macro_rules! Depcrate_ord_mapimpl_190 {
() => {
// Module: crate::ord::map
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'a , K , V > Add for & 'a OrdMap < K , V > where K : Ord + Clone , V : Clone , { type Output = OrdMap < K , V > ; fn add (self , other : Self) -> Self :: Output { self . clone () . union (other . clone ()) } }
};
}
