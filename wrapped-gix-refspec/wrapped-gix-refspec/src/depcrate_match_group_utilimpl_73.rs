// Generated macro for impl_73 (impl)
macro_rules! Depcrate_match_group_utilimpl_73 {
() => {
// Module: crate::match_group::util
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a > From < RefSpecRef < 'a > > for Matcher < 'a > { fn from (v : RefSpecRef < 'a >) -> Self { let mut m = Matcher { lhs : v . src . map (Into :: into) , rhs : v . dst . map (Into :: into) , } ; if m . rhs . is_none () { if let Some (src) = v . src { if must_use_pattern_matching (src) { m . lhs = Some (Needle :: Pattern (src)) ; } } } m } }
};
}
