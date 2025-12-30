// Generated macro for Subo (trait)
macro_rules! Depcrate_int_addsubSubo {
() => {
// Module: crate::int::addsub
// Provides: {"Subo"}
// Dependencies: {}
trait Subo : AddSub where < Self as MinInt > :: Unsigned : UAddSub , { fn subo (self , other : Self) -> (Self , bool) { let sum = AddSub :: sub (self , other) ; (sum , (other < Self :: ZERO) != (self < sum)) } }
};
}
