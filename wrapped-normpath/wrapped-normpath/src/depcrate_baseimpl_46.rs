// Generated macro for impl_46 (impl)
macro_rules! Depcrate_baseimpl_46 {
() => {
// Module: crate::base
// Provides: {"impl_46"}
// Dependencies: {}
impl Deref for BasePathBuf { type Target = BasePath ; # [inline] fn deref (& self) -> & BasePath { BasePath :: from_inner (self . 0 . as_os_str ()) } }
};
}
