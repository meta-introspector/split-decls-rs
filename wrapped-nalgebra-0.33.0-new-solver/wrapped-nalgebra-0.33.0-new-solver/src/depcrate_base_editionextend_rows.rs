// Generated macro for extend_rows (function)
macro_rules! Depcrate_base_editionextend_rows {
() => {
// Module: crate::base::edition
// Provides: {"extend_rows"}
// Dependencies: {}
unsafe fn extend_rows < T > (data : & mut [T] , nrows : usize , ncols : usize , i : usize , ninsert : usize) { let new_nrows = nrows + ninsert ; if new_nrows == 0 || ncols == 0 { return ; } let ptr_in = data . as_ptr () ; let ptr_out = data . as_mut_ptr () ; let remaining_len = nrows - i ; let mut curr_i = new_nrows * ncols - remaining_len ; ptr :: copy (ptr_in . add (nrows * ncols - remaining_len) , ptr_out . add (curr_i) , remaining_len ,) ; for k in (0 .. ncols - 1) . rev () { curr_i -= new_nrows ; ptr :: copy (ptr_in . add (k * nrows + i) , ptr_out . add (curr_i) , nrows) ; } }
};
}
