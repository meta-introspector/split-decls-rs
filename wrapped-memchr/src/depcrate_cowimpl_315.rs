// Generated macro for impl_315 (impl)
macro_rules! Depcrate_cowimpl_315 {
() => {
// Module: crate::cow
// Provides: {"impl_315"}
// Dependencies: {}
impl < 'a > Imp < 'a > { # [inline (always)] pub fn new (bytes : & 'a [u8]) -> Imp < 'a > { # [cfg (feature = "alloc")] { Imp :: Borrowed (bytes) } # [cfg (not (feature = "alloc"))] { Imp (bytes) } } # [cfg (feature = "alloc")] # [inline (always)] pub fn as_slice (& self) -> & [u8] { # [cfg (feature = "alloc")] { match self { Imp :: Owned (ref x) => x , Imp :: Borrowed (x) => x , } } # [cfg (not (feature = "alloc"))] { self . 0 } } # [cfg (not (feature = "alloc"))] # [inline (always)] pub fn as_slice (& self) -> & [u8] { self . 0 } }
};
}
