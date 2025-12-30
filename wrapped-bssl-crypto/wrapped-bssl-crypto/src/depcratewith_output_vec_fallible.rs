// Generated macro for with_output_vec_fallible (function)
macro_rules! Depcratewith_output_vec_fallible {
() => {
// Module: crate
// Provides: {"with_output_vec_fallible"}
// Dependencies: {}
# [doc = " Wrap a closure that writes at most `max_output` bytes to fill a vector."] # [doc = " If successful, it must return the number of bytes written."] # [doc = ""] # [doc = " Safety: `F` must not write more than `max_output` bytes and must return"] # [doc = " the number of bytes written or else return `None` to indicate failure."] unsafe fn with_output_vec_fallible < F > (max_output : usize , func : F) -> Option < Vec < u8 > > where F : FnOnce (* mut u8) -> Option < usize > , { let mut ret = Vec :: with_capacity (max_output) ; let out = ret . spare_capacity_mut () ; let out_buf = out . get_mut (0) . map_or (core :: ptr :: null_mut () , | x | x . as_mut_ptr ()) ; let num_written = func (out_buf) ? ; assert ! (num_written <= ret . capacity ()) ; unsafe { ret . set_len (num_written) ; } Some (ret) }
};
}
