// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1029 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1029"}
// Dependencies: {}
impl < const KEY_FINISHED : bool > RepMatcher < '_ , '_ , KEY_FINISHED > { fn remaining (& self) -> usize { if KEY_FINISHED { self . rep . content . len () - self . forward_cursor } else { self . rep . allowed_upper_bound () - self . forward_cursor } } fn remaining_forward_slice (& self) -> & str { if KEY_FINISHED { & self . rep . as_str () [self . forward_cursor ..] } else { & self . rep . as_str () [self . forward_cursor .. self . rep . allowed_upper_bound ()] } } # [doc = " Returns the index (which is a valid UTF-8 index) of the leftmost matched char in the ante context."] # [inline] fn ante_cursor (& self) -> usize { self . rep . cursor - self . ante_match_len } fn remaining_ante_slice (& self) -> & str { & self . rep . as_str () [.. self . ante_cursor ()] } }
};
}
