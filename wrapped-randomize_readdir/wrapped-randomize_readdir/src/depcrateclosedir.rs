// Generated macro for closedir (function)
macro_rules! Depcrateclosedir {
() => {
// Module: crate
// Provides: {"closedir"}
// Dependencies: {}
# [doc = " Closes a directory stream."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because:"] # [doc = " - `dirp` must be a valid pointer to a directory stream previously opened by `opendir`"] # [doc = " - The directory stream must not be used after calling this function"] # [no_mangle] pub unsafe extern "C" fn closedir (dirp : * mut DIR) -> c_int { info ! ("{:p}: closing handle" , dirp) ; let state = STATE . wait () ; state . dirs . write () . expect ("lock poisoned") . remove (& (dirp as usize)) ; unsafe { (state . closedir) (dirp) } }
};
}
