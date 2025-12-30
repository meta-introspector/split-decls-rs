// Generated macro for impl_18 (impl)
macro_rules! Depcrate_attrsimpl_18 {
() => {
// Module: crate::attrs
// Provides: {"impl_18"}
// Dependencies: {}
impl ops :: Deref for RawAttrs { type Target = [Attr] ; fn deref (& self) -> & [Attr] { match & self . entries { Some (it) => & it . slice , None => & [] , } } }
};
}
