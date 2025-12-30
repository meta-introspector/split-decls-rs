// Generated macro for with_allow_duplicates (function)
macro_rules! Depcrate_runtimewith_allow_duplicates {
() => {
// Module: crate::runtime
// Provides: {"with_allow_duplicates"}
// Dependencies: {}
# [doc = " Helper function to support perfect duplicate detection."] pub fn with_allow_duplicates < R , F > (f : F) -> R where F : FnOnce () -> R , { RECORDED_DUPLICATES . with (| x | x . borrow_mut () . push (BTreeMap :: new ())) ; let rv = std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (f)) ; RECORDED_DUPLICATES . with (| x | x . borrow_mut () . pop () . unwrap ()) ; match rv { Ok (rv) => rv , Err (payload) => std :: panic :: resume_unwind (payload) , } }
};
}
