// Generated macro for impl_91 (impl)
macro_rules! Depcrate_mapimpl_91 {
() => {
// Module: crate::map
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'a , K , V > Entry < 'a , K , V > { fn new (inner : base :: RefEntry < 'a , K , V >) -> Self { Self { inner : ManuallyDrop :: new (inner) , } } # [doc = " Returns a reference to the key."] pub fn key (& self) -> & 'a K { self . inner . key () } # [doc = " Returns a reference to the value."] pub fn value (& self) -> & 'a V { self . inner . value () } # [doc = " Returns `true` if the entry is removed from the map."] pub fn is_removed (& self) -> bool { self . inner . is_removed () } }
};
}
