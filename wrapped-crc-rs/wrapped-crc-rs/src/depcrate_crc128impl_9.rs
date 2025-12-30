// Generated macro for impl_9 (impl)
macro_rules! Depcrate_crc128impl_9 {
() => {
// Module: crate::crc128
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , const L : usize > Digest < 'a , u128 , Table < L > > where Table < L > : private :: Sealed , { const fn new (crc : & 'a Crc < u128 , Table < L > > , value : u128) -> Self { Digest { crc , value } } pub fn update (& mut self , bytes : & [u8]) { self . value = self . crc . update (self . value , bytes) ; } pub const fn finalize (self) -> u128 { finalize (self . crc . algorithm , self . value) } }
};
}
