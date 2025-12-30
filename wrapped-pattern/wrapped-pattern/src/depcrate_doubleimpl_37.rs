// Generated macro for impl_37 (impl)
macro_rules! Depcrate_doubleimpl_37 {
() => {
// Module: crate::double
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > Iterator for DoublePlaceholderPatternIterator < 'a > { type Item = PatternItem < 'a , DoublePlaceholderKey > ; fn next (& mut self) -> Option < Self :: Item > { match self . current_offset . cmp (& self . ph_first . offset) { Ordering :: Less => { let literal_str = match self . store . get (self . current_offset .. self . ph_first . offset) { Some (s) => s , None => { debug_assert ! (false , "offsets are in range") ; "" } } ; self . current_offset = self . ph_first . offset ; Some (PatternItem :: Literal (literal_str)) } Ordering :: Equal => { self . ph_first . offset = 0 ; Some (PatternItem :: Placeholder (self . ph_first . key)) } Ordering :: Greater => match self . current_offset . cmp (& self . ph_second . offset) { Ordering :: Less => { let literal_str = match self . store . get (self . current_offset .. self . ph_second . offset) { Some (s) => s , None => { debug_assert ! (false , "offsets are in range") ; "" } } ; self . current_offset = self . ph_second . offset ; Some (PatternItem :: Literal (literal_str)) } Ordering :: Equal => { self . ph_second . offset = 0 ; Some (PatternItem :: Placeholder (self . ph_second . key)) } Ordering :: Greater => { let literal_str = match self . store . get (self . current_offset ..) { Some (s) => s , None => { debug_assert ! (false , "offsets are in range") ; "" } } ; if literal_str . is_empty () { None } else { self . current_offset = self . store . len () ; Some (PatternItem :: Literal (literal_str)) } } } , } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
