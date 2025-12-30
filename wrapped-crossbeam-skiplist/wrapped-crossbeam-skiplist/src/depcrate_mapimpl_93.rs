// Generated macro for impl_93 (impl)
macro_rules! Depcrate_mapimpl_93 {
() => {
// Module: crate::map
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a , K , V > Entry < 'a , K , V > where K : Ord , { # [doc = " Moves to the next entry in the map."] pub fn move_next (& mut self) -> bool { let guard = & epoch :: pin () ; self . inner . move_next (guard) } # [doc = " Moves to the previous entry in the map."] pub fn move_prev (& mut self) -> bool { let guard = & epoch :: pin () ; self . inner . move_prev (guard) } # [doc = " Returns the next entry in the map."] pub fn next (& self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . next (guard) . map (Entry :: new) } # [doc = " Returns the previous entry in the map."] pub fn prev (& self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . prev (guard) . map (Entry :: new) } }
};
}
