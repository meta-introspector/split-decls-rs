// Generated macro for impl_126 (impl)
macro_rules! Depcrate_setimpl_126 {
() => {
// Module: crate::set
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a , T > Entry < 'a , T > where T : Ord , { # [doc = " Moves to the next entry in the set."] pub fn move_next (& mut self) -> bool { self . inner . move_next () } # [doc = " Moves to the previous entry in the set."] pub fn move_prev (& mut self) -> bool { self . inner . move_prev () } # [doc = " Returns the next entry in the set."] pub fn next (& self) -> Option < Entry < 'a , T > > { self . inner . next () . map (Entry :: new) } # [doc = " Returns the previous entry in the set."] pub fn prev (& self) -> Option < Entry < 'a , T > > { self . inner . prev () . map (Entry :: new) } }
};
}
