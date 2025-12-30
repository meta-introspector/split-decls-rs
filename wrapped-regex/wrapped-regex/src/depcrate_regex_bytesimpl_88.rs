// Generated macro for impl_88 (impl)
macro_rules! Depcrate_regex_bytesimpl_88 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_88"}
// Dependencies: {}
impl < F , T > Replacer for F where F : FnMut (& Captures < '_ >) -> T , T : AsRef < [u8] > , { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { dst . extend_from_slice ((* self) (caps) . as_ref ()) ; } }
};
}
