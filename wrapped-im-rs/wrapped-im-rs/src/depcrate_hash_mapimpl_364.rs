// Generated macro for impl_364 (impl)
macro_rules! Depcrate_hash_mapimpl_364 {
() => {
// Module: crate::hash::map
// Provides: {"impl_364"}
// Dependencies: {}
impl < 'm , 'k , 'v , K , V , OK , OV , SA , SB > From < & 'm HashMap < & 'k K , & 'v V , SA > > for HashMap < OK , OV , SB > where K : Hash + Eq + ToOwned < Owned = OK > + ? Sized , V : ToOwned < Owned = OV > + ? Sized , OK : Hash + Eq + Clone + Borrow < K > , OV : Borrow < V > + Clone , SA : BuildHasher , SB : BuildHasher + Default , { fn from (m : & HashMap < & K , & V , SA >) -> Self { m . iter () . map (| (k , v) | ((* k) . to_owned () , (* v) . to_owned ())) . collect () } }
};
}
