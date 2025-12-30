// Generated macro for impl_47 (impl)
macro_rules! Depcrate_frameimpl_47 {
() => {
// Module: crate::frame
// Provides: {"impl_47"}
// Dependencies: {}
impl Iterator for DisplayFragments < '_ > { type Item = String ; fn next (& mut self) -> Option < Self :: Item > { let mut buf = String :: new () ; self . frame . format_fragment (self . iter . next () ? , & mut buf , & self . frame . args , None) . ok () ? ; Some (buf) } }
};
}
