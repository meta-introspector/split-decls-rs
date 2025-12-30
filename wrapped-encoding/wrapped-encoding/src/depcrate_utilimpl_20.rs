// Generated macro for impl_20 (impl)
macro_rules! Depcrate_utilimpl_20 {
() => {
// Module: crate::util
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'r > StrCharIndex < 'r > for & 'r str { # [doc = " Iterates over each character with corresponding byte offset range."] fn index_iter (& self) -> StrCharIndexIterator < 'r > { StrCharIndexIterator { index : 0 , chars : self . chars () } } }
};
}
