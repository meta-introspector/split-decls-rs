// Generated macro for impl_53 (impl)
macro_rules! Depcrate_crc8impl_53 {
() => {
// Module: crate::crc8
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a , const L : usize > Digest < 'a , u8 , Table < L > > where Table < L > : private :: Sealed , { const fn new (crc : & 'a Crc < u8 , Table < L > > , value : u8) -> Self { Digest { crc , value } } pub fn update (& mut self , bytes : & [u8]) { self . value = self . crc . update (self . value , bytes) ; } pub const fn finalize (self) -> u8 { finalize (self . crc . algorithm , self . value) } }
};
}
