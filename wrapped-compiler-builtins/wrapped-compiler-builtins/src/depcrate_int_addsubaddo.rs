// Generated macro for Addo (trait)
macro_rules! Depcrate_int_addsubAddo {
() => {
// Module: crate::int::addsub
// Provides: {"Addo"}
// Dependencies: {}
trait Addo : AddSub where < Self as MinInt > :: Unsigned : UAddSub , { fn addo (self , other : Self) -> (Self , bool) { let sum = AddSub :: add (self , other) ; (sum , (other < Self :: ZERO) != (sum < self)) } }
};
}
