// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < P : HashParameters > HashCore < P > { fn absorb_block (& mut self , block : & [u8 ; 8]) { self . state [0] ^= u64 :: from_le_bytes (* block) ; self . permute_state () ; } fn absorb_last_block (& mut self , block : & [u8]) { debug_assert ! (block . len () < 8) ; let len = block . len () ; if len > 0 { let mut tmp = [0u8 ; 8] ; tmp [0 .. len] . copy_from_slice (block) ; self . state [0] ^= u64 :: from_le_bytes (tmp) ; } self . state [0] ^= pad (len) ; self . state . permute_12 () ; } fn squeeze (& mut self , mut block : & mut [u8]) { debug_assert_eq ! (block . len () % 8 , 0) ; while block . len () > 8 { block [.. 8] . copy_from_slice (& u64 :: to_le_bytes (self . state [0])) ; self . permute_state () ; block = & mut block [8 ..] ; } block [.. 8] . copy_from_slice (& u64 :: to_le_bytes (self . state [0])) ; } fn squeeze_block (& mut self) -> [u8 ; 8] { let ret = u64 :: to_le_bytes (self . state [0]) ; self . permute_state () ; ret } # [inline (always)] fn permute_state (& mut self) { self . state . permute_12 () ; } }
};
}
