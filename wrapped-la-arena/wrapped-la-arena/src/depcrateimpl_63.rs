// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < T > IntoIterator for Arena < T > { type Item = (Idx < T > , T) ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { IntoIter (self . data . into_iter () . enumerate ()) } }
};
}
