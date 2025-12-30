// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl < C : CmacCipher > FixedOutputCore for CmacCore < C > { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let Self { state , cipher } = self ; let pos = buffer . get_pos () ; let buf = buffer . pad_with_zeros () ; let mut subkey = Default :: default () ; cipher . encrypt_block (& mut subkey) ; let key1 = C :: dbl (subkey) ; xor (state , & buf) ; if pos == buf . len () { xor (state , & key1) ; } else { state [pos] ^= 0x80 ; let key2 = C :: dbl (key1) ; xor (state , & key2) ; } cipher . encrypt_block (state) ; out . copy_from_slice (state) ; } }
};
}
