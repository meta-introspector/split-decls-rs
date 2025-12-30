// Generated macro for impl_308 (impl)
macro_rules! Depcrate_mapimpl_308 {
() => {
// Module: crate::map
// Provides: {"impl_308"}
// Dependencies: {}
impl < K , V , S , A > PartialEq for HashMap < K , V , S , A > where K : Eq + Hash , V : PartialEq , S : BuildHasher , A : Allocator , { fn eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
};
}
