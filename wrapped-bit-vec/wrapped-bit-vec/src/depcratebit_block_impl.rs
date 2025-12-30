// Generated macro for bit_block_impl (macro)
macro_rules! Depcratebit_block_impl {
() => {
// Module: crate
// Provides: {"bit_block_impl"}
// Dependencies: {}
macro_rules ! bit_block_impl { ($ (($ t : ident , $ size : expr)) ,*) => ($ (impl BitBlock for $ t { # [inline] fn bits () -> usize { $ size } # [inline] fn from_byte (byte : u8) -> Self { $ t :: from (byte) } # [inline] fn count_ones (self) -> usize { self . count_ones () as usize } # [inline] fn count_zeros (self) -> usize { self . count_zeros () as usize } # [inline] fn one () -> Self { 1 } # [inline] fn zero () -> Self { 0 } }) *) }
};
}
