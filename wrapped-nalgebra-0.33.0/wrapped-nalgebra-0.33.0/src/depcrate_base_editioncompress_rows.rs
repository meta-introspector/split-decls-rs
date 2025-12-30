// Generated macro for compress_rows (function)
macro_rules! Depcrate_base_editioncompress_rows {
() => {
// Module: crate::base::edition
// Provides: {"compress_rows"}
// Dependencies: {}
unsafe fn compress_rows < T : Scalar > (data : & mut [T] , nrows : usize , ncols : usize , i : usize , nremove : usize ,) { let new_nrows = nrows - nremove ; if nremove == 0 { return ; } if new_nrows == 0 || ncols == 0 { ptr :: drop_in_place (data) ; return ; } let ptr_in = data . as_ptr () ; let ptr_out = data . as_mut_ptr () ; let mut curr_i = i ; for k in 0 .. ncols - 1 { let s = ptr :: slice_from_raw_parts_mut (ptr_out . add (curr_i) , nremove) ; ptr :: drop_in_place (s) ; ptr :: copy (ptr_in . add (curr_i + (k + 1) * nremove) , ptr_out . add (curr_i) , new_nrows ,) ; curr_i += new_nrows ; } let s = ptr :: slice_from_raw_parts_mut (ptr_out . add (curr_i) , nremove) ; ptr :: drop_in_place (s) ; let remaining_len = nrows - i - nremove ; ptr :: copy (ptr_in . add (nrows * ncols - remaining_len) , ptr_out . add (curr_i) , remaining_len ,) ; }
};
}
