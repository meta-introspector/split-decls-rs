// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl < C > FixedOutputCore for BeltMacCore < C > where C : BlockCipherEncrypt + Clone , { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let mut buf = buffer . pad_with_zeros () ; let cipher = & mut self . cipher ; let r = & self . r ; let bs = r . len () ; let mut new_r = Block :: < C > :: default () ; if pos == bs { let (h1 , h2) = new_r . split_at_mut (bs - 4) ; h1 . copy_from_slice (& r [4 ..]) ; for i in 0 .. 4 { h2 [i] = r [i] ^ r [4 + i] ; } } else { buf [pos] = 0x80 ; let (h1 , h2) = new_r . split_at_mut (4) ; for i in 0 .. 4 { h1 [i] = r [i] ^ r [bs - 4 + i] ; } h2 . copy_from_slice (& r [.. bs - 4]) ; } let mut state = self . state . clone () ; xor (& mut state , & buf) ; xor (& mut state , & new_r) ; cipher . encrypt_block_b2b (& state , out) ; } }
};
}
