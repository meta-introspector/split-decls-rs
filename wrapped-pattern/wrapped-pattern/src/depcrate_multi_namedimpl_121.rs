// Generated macro for impl_121 (impl)
macro_rules! Depcrate_multi_namedimpl_121 {
() => {
// Module: crate::multi_named
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'a > MultiNamedPlaceholderPatternIterator < 'a > { fn new (store : & 'a str) -> Self { Self { store } } fn try_next (& mut self ,) -> Result < Option < PatternItem < 'a , MultiNamedPlaceholderKey < 'a > > > , MultiNamedPlaceholderError > { match self . store . find (| x | (x as usize) <= 0x07) { Some (0) => { let Some ((& [lead , trail] , remainder)) = self . store . split_at_checked (2) . map (| (a , b) | (a . as_bytes () , b)) else { return Err (MultiNamedPlaceholderError :: InvalidStore) ; } ; debug_assert ! (lead <= 7) ; if trail > 7 { return Err (MultiNamedPlaceholderError :: InvalidStore) ; } let placeholder_len = (lead << 3) + trail ; let Some ((placeholder_name , remainder)) = remainder . split_at_checked (placeholder_len as usize) else { return Err (MultiNamedPlaceholderError :: InvalidStore) ; } ; self . store = remainder ; Ok (Some (PatternItem :: Placeholder (MultiNamedPlaceholderKey (placeholder_name ,)))) } Some (i) => { let Some ((literal , remainder)) = self . store . split_at_checked (i) else { debug_assert ! (false , "should be a perfect slice") ; return Err (MultiNamedPlaceholderError :: Unreachable) ; } ; self . store = remainder ; Ok (Some (PatternItem :: Literal (literal))) } None if self . store . is_empty () => { Ok (None) } None => { let literal = self . store ; self . store = "" ; Ok (Some (PatternItem :: Literal (literal))) } } } }
};
}
