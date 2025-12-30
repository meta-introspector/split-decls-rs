// Generated macro for impl_163 (impl)
macro_rules! Depcrate_singleimpl_163 {
() => {
// Module: crate::single
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a > Iterator for SinglePlaceholderPatternIterator < 'a > { type Item = PatternItem < 'a , SinglePlaceholderKey > ; fn next (& mut self) -> Option < Self :: Item > { match self . current_offset . cmp (& self . placeholder_offset) { Ordering :: Less => { let literal_str = match self . store . get (self . current_offset .. self . placeholder_offset) { Some (s) => s , None => { debug_assert ! (false , "offsets are in range") ; "" } } ; self . current_offset = self . placeholder_offset ; Some (PatternItem :: Literal (literal_str)) } Ordering :: Equal => { self . placeholder_offset = 0 ; Some (PatternItem :: Placeholder (SinglePlaceholderKey :: Singleton)) } Ordering :: Greater => { let literal_str = match self . store . get (self . current_offset ..) { Some (s) => s , None => { debug_assert ! (false , "offsets are in range") ; "" } } ; if literal_str . is_empty () { None } else { self . current_offset = self . store . len () ; Some (PatternItem :: Literal (literal_str)) } } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
