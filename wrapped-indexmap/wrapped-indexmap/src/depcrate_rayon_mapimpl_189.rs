// Generated macro for impl_189 (impl)
macro_rules! Depcrate_rayon_mapimpl_189 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_189"}
// Dependencies: {}
impl < K , V , S > IndexMap < K , V , S > where K : Hash + Eq + Sync , V : Sync , S : BuildHasher , { # [doc = " Returns `true` if `self` contains all of the same key-value pairs as `other`,"] # [doc = " regardless of each map's indexed order, determined in parallel."] pub fn par_eq < V2 , S2 > (& self , other : & IndexMap < K , V2 , S2 >) -> bool where V : PartialEq < V2 > , V2 : Sync , S2 : BuildHasher + Sync , { self . len () == other . len () && self . par_iter () . all (move | (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
};
}
