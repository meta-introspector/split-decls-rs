// Generated macro for impl_563 (impl)
macro_rules! Depcrate_uriimpl_563 {
() => {
// Module: crate::uri
// Provides: {"impl_563"}
// Dependencies: {}
impl PartialEq for Uri { fn eq (& self , other : & Uri) -> bool { if self . scheme () != other . scheme () { return false ; } if self . authority () != other . authority () { return false ; } if self . path () != other . path () { return false ; } if self . query () != other . query () { return false ; } true } }
};
}
