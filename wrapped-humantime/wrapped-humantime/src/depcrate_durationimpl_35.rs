// Generated macro for impl_35 (impl)
macro_rules! Depcrate_durationimpl_35 {
() => {
// Module: crate::duration
// Provides: {"impl_35"}
// Dependencies: {}
impl OverflowOp for u64 { fn mul (self , other : Self) -> Result < Self , Error > { self . checked_mul (other) . ok_or (Error :: NumberOverflow) } fn add (self , other : Self) -> Result < Self , Error > { self . checked_add (other) . ok_or (Error :: NumberOverflow) } fn div (self , other : Self) -> Result < Self , Error > { match self % other { 0 => Ok (self / other) , _ => Err (Error :: NumberOverflow) , } } }
};
}
