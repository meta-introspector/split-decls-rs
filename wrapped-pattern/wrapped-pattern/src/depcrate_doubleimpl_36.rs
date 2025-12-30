// Generated macro for impl_36 (impl)
macro_rules! Depcrate_doubleimpl_36 {
() => {
// Module: crate::double
// Provides: {"impl_36"}
// Dependencies: {}
impl ExactSizeIterator for DoublePlaceholderPatternIterator < '_ > { fn len (& self) -> usize { let mut chars = self . store . chars () ; let (mut ph_first , ph_first_len) = match chars . next () { Some (ch) => (DoublePlaceholderInfo :: from_char (ch) , ch . len_utf8 ()) , None => { debug_assert ! (false) ; (DoublePlaceholderInfo :: no_place0 () , 0) } } ; let (mut ph_second , ph_second_len) = match chars . next () { Some (ch) => (DoublePlaceholderInfo :: from_char (ch) , ch . len_utf8 ()) , None => { debug_assert ! (false) ; (ph_first . swap () , ph_first_len) } } ; let initial_offset = ph_first_len + ph_second_len ; ph_first . offset += initial_offset - 1 ; ph_second . offset += initial_offset - 1 ; let store_len = self . store . len () ; # [expect (clippy :: comparison_chain)] if ph_first . offset < initial_offset { if initial_offset < store_len { 1 } else { 0 } } else if ph_first . offset == initial_offset { if ph_second . offset < initial_offset { if ph_first . offset < store_len { 2 } else { 1 } } else if ph_second . offset == ph_first . offset { if ph_first . offset < store_len { 3 } else { 2 } } else if ph_second . offset < store_len { 4 } else { 3 } } else { if ph_second . offset < initial_offset { if ph_first . offset < store_len { 3 } else { 2 } } else if ph_second . offset == ph_first . offset { if ph_first . offset < store_len { 4 } else { 3 } } else if ph_second . offset < store_len { 5 } else { 4 } } } }
};
}
