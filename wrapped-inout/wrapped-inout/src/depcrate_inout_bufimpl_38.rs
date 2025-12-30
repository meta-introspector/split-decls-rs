// Generated macro for impl_38 (impl)
macro_rules! Depcrate_inout_bufimpl_38 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_38"}
// Dependencies: {}
impl InOutBuf < '_ , '_ , u8 > { # [doc = " XORs `data` with values behind the input slice and write"] # [doc = " result to the output slice."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `data` length is not equal to the buffer length."] # [inline (always)] # [allow (clippy :: needless_range_loop)] pub fn xor_in2out (& mut self , data : & [u8]) { assert_eq ! (self . len () , data . len ()) ; unsafe { for i in 0 .. data . len () { let in_ptr = self . in_ptr . add (i) ; let out_ptr = self . out_ptr . add (i) ; * out_ptr = * in_ptr ^ data [i] ; } } } }
};
}
