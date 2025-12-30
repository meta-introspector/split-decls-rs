// Generated macro for impl_674 (impl)
macro_rules! Depcrate_teeimpl_674 {
() => {
// Module: crate::tee
// Provides: {"impl_674"}
// Dependencies: {}
impl < I > Iterator for Tee < I > where I : Iterator , I :: Item : Clone , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let mut buffer = self . rcbuffer . borrow_mut () ; if buffer . owner == self . id { match buffer . backlog . pop_front () { None => { } some_elt => return some_elt , } } match buffer . iter . next () { None => None , Some (elt) => { buffer . backlog . push_back (elt . clone ()) ; buffer . owner = ! self . id ; Some (elt) } } } fn size_hint (& self) -> (usize , Option < usize >) { let buffer = self . rcbuffer . borrow () ; let sh = buffer . iter . size_hint () ; if buffer . owner == self . id { let log_len = buffer . backlog . len () ; size_hint :: add_scalar (sh , log_len) } else { sh } } }
};
}
