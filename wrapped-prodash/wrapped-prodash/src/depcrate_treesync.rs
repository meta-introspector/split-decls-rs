// Generated macro for sync (module)
macro_rules! Depcrate_treesync {
() => {
// Module: crate::tree
// Provides: {"sync"}
// Dependencies: {}
# [cfg (not (feature = "dashmap"))] pub (crate) mod sync { pub struct HashMap < K , V > (parking_lot :: Mutex < std :: collections :: HashMap < K , V > >) ; impl < K , V > HashMap < K , V > where K : Eq + std :: hash :: Hash , { pub fn with_capacity (cap : usize) -> Self { HashMap (parking_lot :: Mutex :: new (std :: collections :: HashMap :: with_capacity (cap))) } pub fn extend_to (& self , out : & mut Vec < (K , V) >) where K : Clone , V : Clone , { let lock = self . 0 . lock () ; out . extend (lock . iter () . map (| (k , v) | (k . clone () , v . clone ()))) } pub fn remove (& self , key : & K) -> Option < V > { self . 0 . lock () . remove (key) } pub fn get < T > (& self , key : & K , cb : impl FnOnce (& V) -> T) -> Option < T > { self . 0 . lock () . get (key) . map (cb) } pub fn get_mut < T > (& self , key : & K , cb : impl FnOnce (& mut V) -> T) -> Option < T > { self . 0 . lock () . get_mut (key) . map (cb) } pub fn insert (& self , key : K , value : V) { self . 0 . lock () . insert (key , value) ; } pub fn len (& self) -> usize { self . 0 . lock () . len () } pub fn clone (& self) -> Self where K : Clone , V : Clone , { HashMap (parking_lot :: Mutex :: new (self . 0 . lock () . clone ())) } } }
};
}
