// Generated macro for Matcher (struct)
macro_rules! Depcrate_match_group_utilMatcher {
() => {
// Module: crate::match_group::util
// Provides: {"Matcher"}
// Dependencies: {}
# [doc = " A type keeping enough information about a ref-spec to be able to efficiently match it against multiple matcher items."] # [derive (Debug)] pub struct Matcher < 'a > { pub (crate) lhs : Option < Needle < 'a > > , pub (crate) rhs : Option < Needle < 'a > > , }
};
}
