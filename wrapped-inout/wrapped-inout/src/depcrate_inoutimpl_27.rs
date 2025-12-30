// Generated macro for impl_27 (impl)
macro_rules! Depcrate_inoutimpl_27 {
() => {
// Module: crate::inout
// Provides: {"impl_27"}
// Dependencies: {}
impl < N : ArraySize > InOut < '_ , '_ , Array < u8 , N > > { # [doc = " XOR `data` with values behind the input slice and write"] # [doc = " result to the output slice."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `data` length is not equal to the buffer length."] # [inline (always)] # [allow (clippy :: needless_range_loop)] pub fn xor_in2out (& mut self , data : & Array < u8 , N >) { unsafe { let input = ptr :: read (self . in_ptr) ; let mut temp = Array :: < u8 , N > :: default () ; for i in 0 .. N :: USIZE { temp [i] = input [i] ^ data [i] ; } ptr :: write (self . out_ptr , temp) ; } } }
};
}
