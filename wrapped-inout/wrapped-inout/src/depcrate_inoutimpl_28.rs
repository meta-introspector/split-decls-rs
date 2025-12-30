// Generated macro for impl_28 (impl)
macro_rules! Depcrate_inoutimpl_28 {
() => {
// Module: crate::inout
// Provides: {"impl_28"}
// Dependencies: {}
impl < N , M > InOut < '_ , '_ , Array < Array < u8 , N > , M > > where N : ArraySize , M : ArraySize , { # [doc = " XOR `data` with values behind the input slice and write"] # [doc = " result to the output slice."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `data` length is not equal to the buffer length."] # [inline (always)] # [allow (clippy :: needless_range_loop)] pub fn xor_in2out (& mut self , data : & Array < Array < u8 , N > , M >) { unsafe { let input = ptr :: read (self . in_ptr) ; let mut temp = Array :: < Array < u8 , N > , M > :: default () ; for i in 0 .. M :: USIZE { for j in 0 .. N :: USIZE { temp [i] [j] = input [i] [j] ^ data [i] [j] ; } } ptr :: write (self . out_ptr , temp) ; } } }
};
}
