// Generated macro for impl_178 (impl)
macro_rules! Depcrate_ord_mapimpl_178 {
() => {
// Module: crate::ord::map
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , K , V > OccupiedEntry < 'a , K , V > where K : 'a + Ord + Clone , V : 'a + Clone , { # [doc = " Get the key for this entry."] # [must_use] pub fn key (& self) -> & K { & self . key } # [doc = " Remove this entry from the map and return the removed mapping."] pub fn remove_entry (self) -> (K , V) { self . map . remove_with_key (& self . key) . expect ("ordmap::OccupiedEntry::remove_entry: key has vanished!") } # [doc = " Get the current value."] # [must_use] pub fn get (& self) -> & V { self . map . get (& self . key) . unwrap () } # [doc = " Get a mutable reference to the current value."] # [must_use] pub fn get_mut (& mut self) -> & mut V { self . map . get_mut (& self . key) . unwrap () } # [doc = " Convert this entry into a mutable reference."] # [must_use] pub fn into_mut (self) -> & 'a mut V { self . map . get_mut (& self . key) . unwrap () } # [doc = " Overwrite the current value."] pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } # [doc = " Remove this entry from the map and return the removed value."] pub fn remove (self) -> V { self . remove_entry () . 1 } }
};
}
