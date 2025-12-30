// Generated macro for impl_20 (impl)
macro_rules! Depcrate_crc16impl_20 {
() => {
// Module: crate::crc16
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , const L : usize > Digest < 'a , u16 , Table < L > > where Table < L > : private :: Sealed , { const fn new (crc : & 'a Crc < u16 , Table < L > > , value : u16) -> Self { Digest { crc , value } } pub fn update (& mut self , bytes : & [u8]) { self . value = self . crc . update (self . value , bytes) ; } pub const fn finalize (self) -> u16 { finalize (self . crc . algorithm , self . value) } }
};
}
