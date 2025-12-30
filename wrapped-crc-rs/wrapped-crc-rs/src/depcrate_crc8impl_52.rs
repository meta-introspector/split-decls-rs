// Generated macro for impl_52 (impl)
macro_rules! Depcrate_crc8impl_52 {
() => {
// Module: crate::crc8
// Provides: {"impl_52"}
// Dependencies: {}
impl < const L : usize > Crc < u8 , Table < L > > where Table < L > : private :: Sealed , { pub const fn new (algorithm : & 'static Algorithm < u8 >) -> Self { Self { algorithm , data : crc8_table (algorithm . width , algorithm . poly , algorithm . refin) , } } pub const fn checksum (& self , bytes : & [u8]) -> u8 { let mut crc = init (self . algorithm , self . algorithm . init) ; crc = self . update (crc , bytes) ; finalize (self . algorithm , crc) } const fn update (& self , crc : u8 , bytes : & [u8]) -> u8 { update_table (crc , self . algorithm , & self . data , bytes) } pub const fn digest (& self) -> Digest < u8 , Table < L > > { self . digest_with_initial (self . algorithm . init) } # [doc = " Construct a `Digest` with a given initial value."] # [doc = ""] # [doc = " This overrides the initial value specified by the algorithm."] # [doc = " The effects of the algorithm's properties `refin` and `width`"] # [doc = " are applied to the custom initial value."] pub const fn digest_with_initial (& self , initial : u8) -> Digest < u8 , Table < L > > { let value = init (self . algorithm , initial) ; Digest :: new (self , value) } pub const fn table (& self) -> & < Table < L > as Implementation > :: Data < u8 > { & self . data } }
};
}
