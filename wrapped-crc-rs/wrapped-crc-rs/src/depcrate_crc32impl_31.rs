// Generated macro for impl_31 (impl)
macro_rules! Depcrate_crc32impl_31 {
() => {
// Module: crate::crc32
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , const L : usize > Digest < 'a , u32 , Table < L > > where Table < L > : private :: Sealed , { const fn new (crc : & 'a Crc < u32 , Table < L > > , value : u32) -> Self { Digest { crc , value } } pub fn update (& mut self , bytes : & [u8]) { self . value = self . crc . update (self . value , bytes) ; } pub const fn finalize (self) -> u32 { finalize (self . crc . algorithm , self . value) } }
};
}
