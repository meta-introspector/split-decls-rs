// Generated macro for impl_8 (impl)
macro_rules! Depcrate_crc128impl_8 {
() => {
// Module: crate::crc128
// Provides: {"impl_8"}
// Dependencies: {}
impl < const L : usize > Crc < u128 , Table < L > > where Table < L > : private :: Sealed , { pub const fn new (algorithm : & 'static Algorithm < u128 >) -> Self { Self { algorithm , data : crc128_table (algorithm . width , algorithm . poly , algorithm . refin) , } } pub const fn checksum (& self , bytes : & [u8]) -> u128 { let mut crc = init (self . algorithm , self . algorithm . init) ; crc = self . update (crc , bytes) ; finalize (self . algorithm , crc) } const fn update (& self , crc : u128 , bytes : & [u8]) -> u128 { update_table (crc , self . algorithm , & self . data , bytes) } pub const fn digest (& self) -> Digest < u128 , Table < L > > { self . digest_with_initial (self . algorithm . init) } # [doc = " Construct a `Digest` with a given initial value."] # [doc = ""] # [doc = " This overrides the initial value specified by the algorithm."] # [doc = " The effects of the algorithm's properties `refin` and `width`"] # [doc = " are applied to the custom initial value."] pub const fn digest_with_initial (& self , initial : u128) -> Digest < u128 , Table < L > > { let value = init (self . algorithm , initial) ; Digest :: new (self , value) } pub const fn table (& self) -> & < Table < L > as Implementation > :: Data < u128 > { & self . data } }
};
}
