// Generated macro for Needle (enum)
macro_rules! Depcrate_match_group_utilNeedle {
() => {
// Module: crate::match_group::util
// Provides: {"Needle"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] pub (crate) enum Needle < 'a > { FullName (& 'a BStr) , PartialName (& 'a BStr) , Glob { name : & 'a BStr , asterisk_pos : usize } , Pattern (& 'a BStr) , Object (ObjectId) , }
};
}
