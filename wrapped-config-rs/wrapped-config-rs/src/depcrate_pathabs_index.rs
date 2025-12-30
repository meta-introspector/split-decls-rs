// Generated macro for abs_index (function)
macro_rules! Depcrate_pathabs_index {
() => {
// Module: crate::path
// Provides: {"abs_index"}
// Dependencies: {}
# [doc = " Convert a relative index into an absolute index"] fn abs_index (index : isize , len : usize) -> Result < usize , usize > { if index >= 0 { Ok (index as usize) } else if let Some (index) = len . checked_sub (index . unsigned_abs ()) { Ok (index) } else { Err ((len as isize + index) . unsigned_abs ()) } }
};
}
