// Generated macro for gz_skip (function)
macro_rules! Depcrate_gzgz_skip {
() => {
// Module: crate::gz
// Provides: {"gz_skip"}
// Dependencies: {}
fn gz_skip (state : & mut GzState , mut len : i64) -> Result < () , () > { while len != 0 { if state . have != 0 { let n = if gt_off ! (state . have) || state . have as i64 > len { len as usize } else { state . have as usize } ; state . have -= n as c_uint ; state . next = unsafe { state . next . add (n) } ; state . pos += n as i64 ; len -= n as i64 ; } else if state . eof && state . stream . avail_in == 0 { break ; } else { if unsafe { gz_fetch (state) } . is_err () { return Err (()) ; } } } Ok (()) }
};
}
