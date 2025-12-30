// Generated macro for impl_42 (impl)
macro_rules! Depcrate_crc64impl_42 {
() => {
// Module: crate::crc64
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , const L : usize > Digest < 'a , u64 , Table < L > > where Table < L > : private :: Sealed , { const fn new (crc : & 'a Crc < u64 , Table < L > > , value : u64) -> Self { Digest { crc , value } } pub fn update (& mut self , bytes : & [u8]) { self . value = self . crc . update (self . value , bytes) ; } pub const fn finalize (self) -> u64 { finalize (self . crc . algorithm , self . value) } }
};
}
