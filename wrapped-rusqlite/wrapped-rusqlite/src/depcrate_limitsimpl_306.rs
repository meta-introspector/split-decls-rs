// Generated macro for impl_306 (impl)
macro_rules! Depcrate_limitsimpl_306 {
() => {
// Module: crate::limits
// Provides: {"impl_306"}
// Dependencies: {}
impl Connection { # [doc = " Returns the current value of a [`Limit`]."] # [inline] pub fn limit (& self , limit : Limit) -> Result < i32 > { let c = self . db . borrow () ; let rc = unsafe { ffi :: sqlite3_limit (c . db () , limit as c_int , - 1) } ; if rc < 0 { return Err (err ! (ffi :: SQLITE_RANGE , "{limit:?} is invalid")) ; } Ok (rc) } # [doc = " Changes the [`Limit`] to `new_val`, returning the prior"] # [doc = " value of the limit."] # [inline] pub fn set_limit (& self , limit : Limit , new_val : i32) -> Result < i32 > { if new_val < 0 { return Err (err ! (ffi :: SQLITE_RANGE , "{new_val} is invalid")) ; } let c = self . db . borrow_mut () ; let rc = unsafe { ffi :: sqlite3_limit (c . db () , limit as c_int , new_val) } ; if rc < 0 { return Err (err ! (ffi :: SQLITE_RANGE , "{limit:?} is invalid")) ; } Ok (rc) } }
};
}
