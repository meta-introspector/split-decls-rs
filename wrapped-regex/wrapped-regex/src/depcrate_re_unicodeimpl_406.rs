// Generated macro for impl_406 (impl)
macro_rules! Depcrate_re_unicodeimpl_406 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_406"}
// Dependencies: {}
impl < 'r , 't > FindMatches < 'r , 't > { fn text (& self) -> & 't str { match self . 0 { FindMatchesInner :: Dynamic (ref it) => it . text () , FindMatchesInner :: Plugin (ref it) => it . text () , } } }
};
}
