// Generated macro for impl_827 (impl)
macro_rules! Depcrate_ir_immediatesimpl_827 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_827"}
// Dependencies: {}
impl Offset32 { # [doc = " Create a new `Offset32` representing the signed number `x`."] pub fn new (x : i32) -> Self { Self (x) } # [doc = " Create a new `Offset32` representing the signed number `x` if possible."] pub fn try_from_i64 (x : i64) -> Option < Self > { let x = i32 :: try_from (x) . ok () ? ; Some (Self :: new (x)) } # [doc = " Add in the signed number `x` if possible."] pub fn try_add_i64 (self , x : i64) -> Option < Self > { let x = i32 :: try_from (x) . ok () ? ; let ret = self . 0 . checked_add (x) ? ; Some (Self :: new (ret)) } }
};
}
