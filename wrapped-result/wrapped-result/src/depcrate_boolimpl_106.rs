// Generated macro for impl_106 (impl)
macro_rules! Depcrate_boolimpl_106 {
() => {
// Module: crate::bool
// Provides: {"impl_106"}
// Dependencies: {}
impl BOOL { # [doc = " Converts the [`BOOL`] to a [`prim@bool`] value."] # [inline] pub fn as_bool (self) -> bool { self . 0 != 0 } # [doc = " Converts the [`BOOL`] to [`Result<()>`][Result<_>]."] # [inline] pub fn ok (self) -> Result < () > { if self . as_bool () { Ok (()) } else { Err (Error :: from_thread ()) } } # [doc = " Asserts that `self` is a success code."] # [inline] # [track_caller] pub fn unwrap (self) { self . ok () . unwrap () ; } # [doc = " Asserts that `self` is a success code using the given panic message."] # [inline] # [track_caller] pub fn expect (self , msg : & str) { self . ok () . expect (msg) ; } }
};
}
