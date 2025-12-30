// Generated macro for impl_79 (impl)
macro_rules! Depcrate_search_refmapimpl_79 {
() => {
// Module: crate::search::refmap
// Provides: {"impl_79"}
// Dependencies: {}
impl < T > RefMap < T > where T : Hash + Clone , { pub (crate) fn len (& self) -> usize { self . 0 . len () } pub (crate) fn insert (& mut self , value : & T) -> RefMapKey { let mut s = DefaultHasher :: new () ; value . hash (& mut s) ; let key = s . finish () ; match self . 0 . entry (key) { Entry :: Vacant (e) => { e . insert (value . clone ()) ; key } Entry :: Occupied (_) => key , } } pub (crate) fn insert_owned (& mut self , value : T) -> RefMapKey { let mut s = DefaultHasher :: new () ; value . hash (& mut s) ; let key = s . finish () ; match self . 0 . entry (key) { Entry :: Vacant (e) => { e . insert (value) ; key } Entry :: Occupied (_) => key , } } pub (crate) fn resolve (& self , key : RefMapKey) -> Option < & T > { self . 0 . get (& key) } }
};
}
