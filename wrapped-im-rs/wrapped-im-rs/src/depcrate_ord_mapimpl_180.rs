// Generated macro for impl_180 (impl)
macro_rules! Depcrate_ord_mapimpl_180 {
() => {
// Module: crate::ord::map
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'a , K , V > VacantEntry < 'a , K , V > where K : 'a + Ord + Clone , V : 'a + Clone , { # [doc = " Get the key for this entry."] # [must_use] pub fn key (& self) -> & K { & self . key } # [doc = " Convert this entry into its key."] # [must_use] pub fn into_key (self) -> K { self . key } # [doc = " Insert a value into this entry."] pub fn insert (self , value : V) -> & 'a mut V { self . map . insert (self . key . clone () , value) ; self . map . get_mut (& self . key) . unwrap () } }
};
}
