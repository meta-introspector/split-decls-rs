// Generated macro for gz_reset (function)
macro_rules! Depcrate_gzgz_reset {
() => {
// Module: crate::gz
// Provides: {"gz_reset"}
// Dependencies: {}
fn gz_reset (state : & mut GzState) { state . have = 0 ; if state . mode == GzMode :: GZ_READ { state . eof = false ; state . past = false ; state . how = How :: Look ; } else { state . reset = false ; } state . seek = false ; unsafe { gz_error (state , None) } ; state . pos = 0 ; state . stream . avail_in = 0 ; }
};
}
