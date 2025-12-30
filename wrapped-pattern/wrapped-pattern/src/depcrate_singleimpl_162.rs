// Generated macro for impl_162 (impl)
macro_rules! Depcrate_singleimpl_162 {
() => {
// Module: crate::single
// Provides: {"impl_162"}
// Dependencies: {}
impl ExactSizeIterator for SinglePlaceholderPatternIterator < '_ > { fn len (& self) -> usize { let placeholder_offset_char = match self . store . chars () . next () { Some (i) => i , None => { debug_assert ! (false) ; '\0' } } ; let initial_offset = placeholder_offset_char . len_utf8 () ; let placeholder_offset = placeholder_offset_char as usize + initial_offset - 1 ; let store_len = self . store . len () ; if placeholder_offset < initial_offset { if initial_offset < store_len { 1 } else { 0 } } else if placeholder_offset == initial_offset { if initial_offset < store_len { 2 } else { 1 } } else if placeholder_offset < store_len { 3 } else { 2 } } }
};
}
