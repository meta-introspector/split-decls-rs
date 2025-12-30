// Generated macro for impl_64 (impl)
macro_rules! Depcrate_search_outcomeimpl_64 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_64"}
// Dependencies: {}
impl std :: fmt :: Debug for Outcome { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { struct AsDisplay < 'a > (& 'a dyn std :: fmt :: Display) ; impl std :: fmt :: Debug for AsDisplay < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } } let mut dbg = f . debug_tuple ("Outcome") ; if self . selected . is_empty () { for match_ in self . iter () { dbg . field (& AsDisplay (& match_ . assignment)) ; } } else { for match_ in self . iter_selected () { dbg . field (& AsDisplay (& match_ . assignment)) ; } } dbg . finish () } }
};
}
