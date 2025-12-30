// Generated macro for impl_92 (impl)
macro_rules! Depcrate_regex_bytesimpl_92 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_92"}
// Dependencies: {}
impl < 's > Replacer for NoExpand < 's > { fn replace_append (& mut self , _ : & Captures < '_ > , dst : & mut Vec < u8 >) { dst . extend_from_slice (self . 0) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { Some (Cow :: Borrowed (self . 0)) } }
};
}
