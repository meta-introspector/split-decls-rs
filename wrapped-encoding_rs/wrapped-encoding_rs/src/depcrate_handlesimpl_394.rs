// Generated macro for impl_394 (impl)
macro_rules! Depcrate_handlesimpl_394 {
() => {
// Module: crate::handles
// Provides: {"impl_394"}
// Dependencies: {}
impl < 'a , 'b > ByteReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut ByteSource < 'b >) -> ByteReadHandle < 'a , 'b > { ByteReadHandle { source : src } } # [inline (always)] pub fn read (self) -> (u8 , ByteUnreadHandle < 'a , 'b >) { let byte = self . source . read () ; let handle = ByteUnreadHandle :: new (self . source) ; (byte , handle) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
};
}
