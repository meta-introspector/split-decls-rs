// Generated macro for impl_191 (impl)
macro_rules! Depcrate_ord_mapimpl_191 {
() => {
// Module: crate::ord::map
// Provides: {"impl_191"}
// Dependencies: {}
impl < K , V > Add for OrdMap < K , V > where K : Ord + Clone , V : Clone , { type Output = OrdMap < K , V > ; fn add (self , other : Self) -> Self :: Output { self . union (other) } }
};
}
