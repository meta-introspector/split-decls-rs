// Generated macro for impl_18 (impl)
macro_rules! Depcrate_utilimpl_18 {
() => {
// Module: crate::util
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'r > Iterator for StrCharIndexIterator < 'r > { type Item = ((usize , usize) , char) ; # [inline] fn next (& mut self) -> Option < ((usize , usize) , char) > { if let Some (ch) = self . chars . next () { let prev = self . index ; let next = prev + ch . len_utf8 () ; self . index = next ; Some (((prev , next) , ch)) } else { None } } }
};
}
