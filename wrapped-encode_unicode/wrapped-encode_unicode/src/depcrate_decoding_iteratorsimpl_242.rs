// Generated macro for impl_242 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_242 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_242"}
// Dependencies: {}
impl < B : Borrow < u8 > , I : Iterator < Item = B > + Debug > Debug for Utf8CharMerger < B , I > { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { let mut in_order = [0u8 ; 3] ; for i in 0 .. self . after_err_leftover as usize { in_order [i] = self . after_err_stack [self . after_err_leftover as usize - i - 1] ; } fmtr . debug_struct ("Utf8CharMerger") . field ("buffered" , & & in_order [.. self . after_err_leftover as usize]) . field ("inner" , & self . iter) . finish () } }
};
}
