// Generated macro for impl_314 (impl)
macro_rules! Depcrate_cowimpl_314 {
() => {
// Module: crate::cow
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'a > CowBytes < 'a > { # [doc = " Create a new borrowed CowBytes."] # [inline (always)] pub (crate) fn new < B : ? Sized + AsRef < [u8] > > (bytes : & 'a B) -> CowBytes < 'a > { CowBytes (Imp :: new (bytes . as_ref ())) } # [doc = " Create a new owned CowBytes."] # [cfg (feature = "alloc")] # [inline (always)] fn new_owned (bytes : alloc :: boxed :: Box < [u8] >) -> CowBytes < 'static > { CowBytes (Imp :: Owned (bytes)) } # [doc = " Return a borrowed byte string, regardless of whether this is an owned"] # [doc = " or borrowed byte string internally."] # [inline (always)] pub (crate) fn as_slice (& self) -> & [u8] { self . 0 . as_slice () } # [doc = " Return an owned version of this copy-on-write byte string."] # [doc = ""] # [doc = " If this is already an owned byte string internally, then this is a"] # [doc = " no-op. Otherwise, the internal byte string is copied."] # [cfg (feature = "alloc")] # [inline (always)] pub (crate) fn into_owned (self) -> CowBytes < 'static > { match self . 0 { Imp :: Borrowed (b) => { CowBytes :: new_owned (alloc :: boxed :: Box :: from (b)) } Imp :: Owned (b) => CowBytes :: new_owned (b) , } } }
};
}
