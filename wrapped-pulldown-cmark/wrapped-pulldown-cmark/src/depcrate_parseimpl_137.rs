// Generated macro for impl_137 (impl)
macro_rules! Depcrate_parseimpl_137 {
() => {
// Module: crate::parse
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'input > ParserInner < 'input > { fn next_event_range (& mut self , mut broken_link_callback : Option < & mut dyn BrokenLinkCallback < 'input > > ,) -> Option < (Event < 'input > , Range < usize >) > { match self . tree . cur () { None => { let ix = self . tree . pop () ? ; let ix = if matches ! (self . tree [ix] . item . body , ItemBody :: TightParagraph) { self . tree . next_sibling (ix) ; return self . next_event_range (broken_link_callback) ; } else { ix } ; let tag_end = body_to_tag_end (& self . tree [ix] . item . body) ; self . tree . next_sibling (ix) ; let span = self . tree [ix] . item . start .. self . tree [ix] . item . end ; debug_assert ! (span . start <= span . end) ; Some ((Event :: End (tag_end) , span)) } Some (cur_ix) => { let cur_ix = if matches ! (self . tree [cur_ix] . item . body , ItemBody :: TightParagraph) { self . tree . push () ; self . tree . cur () . unwrap () } else { cur_ix } ; if self . tree [cur_ix] . item . body . is_maybe_inline () { self . handle_inline (& mut broken_link_callback) ; } let node = self . tree [cur_ix] ; let item = node . item ; let event = item_to_event (item , self . text , & mut self . allocs) ; if let Event :: Start (..) = event { self . tree . push () ; } else { self . tree . next_sibling (cur_ix) ; } debug_assert ! (item . start <= item . end) ; Some ((event , item . start .. item . end)) } } } }
};
}
