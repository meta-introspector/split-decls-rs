// Generated macro for impl_110 (impl)
macro_rules! Depcrate_parseimpl_110 {
() => {
// Module: crate::parse
// Provides: {"impl_110"}
// Dependencies: {}
impl MathDelims { fn new () -> Self { Self { inner : Default :: default () , } } fn insert (& mut self , delim_is_display : bool , brace_context : u8 , ix : TreeIndex , can_close : bool ,) { self . inner . entry (brace_context) . or_default () . push_back ((ix , can_close , delim_is_display)) ; } fn is_populated (& self) -> bool { ! self . inner . is_empty () } fn find (& mut self , tree : & Tree < Item > , open_ix : TreeIndex , is_display : bool , brace_context : u8 ,) -> Option < TreeIndex > { while let Some ((ix , can_close , delim_is_display)) = self . inner . get_mut (& brace_context) ? . pop_front () { if ix <= open_ix || (is_display && tree [open_ix] . next == Some (ix)) { continue ; } let can_close = can_close && tree [open_ix] . item . end != tree [ix] . item . start ; if (! is_display && can_close) || (is_display && delim_is_display) { return Some (ix) ; } self . inner . get_mut (& brace_context) ? . push_front ((ix , can_close , delim_is_display)) ; break ; } None } fn clear (& mut self) { self . inner . clear () ; } }
};
}
