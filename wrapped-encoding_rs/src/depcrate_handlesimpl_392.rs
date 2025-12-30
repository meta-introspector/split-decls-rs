// Generated macro for impl_392 (impl)
macro_rules! Depcrate_handlesimpl_392 {
() => {
// Module: crate::handles
// Provides: {"impl_392"}
// Dependencies: {}
impl < 'a > ByteSource < 'a > { # [inline (always)] pub fn new (src : & [u8]) -> ByteSource { ByteSource { slice : src , pos : 0 } } # [inline (always)] pub fn check_available < 'b > (& 'b mut self) -> Space < ByteReadHandle < 'b , 'a > > { if self . pos < self . slice . len () { Space :: Available (ByteReadHandle :: new (self)) } else { Space :: Full (self . consumed ()) } } # [inline (always)] fn read (& mut self) -> u8 { let ret = self . slice [self . pos] ; self . pos += 1 ; ret } # [inline (always)] fn unread (& mut self) -> usize { self . pos -= 1 ; self . pos } # [inline (always)] pub fn consumed (& self) -> usize { self . pos } }
};
}
