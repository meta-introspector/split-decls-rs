// Generated macro for impl_396 (impl)
macro_rules! Depcrate_handlesimpl_396 {
() => {
// Module: crate::handles
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'a , 'b > ByteUnreadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut ByteSource < 'b >) -> ByteUnreadHandle < 'a , 'b > { ByteUnreadHandle { source : src } } # [inline (always)] pub fn unread (self) -> usize { self . source . unread () } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } # [inline (always)] pub fn commit (self) -> & 'a mut ByteSource < 'b > { self . source } }
};
}
