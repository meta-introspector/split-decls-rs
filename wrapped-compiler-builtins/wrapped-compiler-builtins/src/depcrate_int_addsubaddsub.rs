// Generated macro for AddSub (trait)
macro_rules! Depcrate_int_addsubAddSub {
() => {
// Module: crate::int::addsub
// Provides: {"AddSub"}
// Dependencies: {}
trait AddSub : Int where < Self as MinInt > :: Unsigned : UAddSub , { fn add (self , other : Self) -> Self { Self :: from_unsigned (self . unsigned () . uadd (other . unsigned ())) } fn sub (self , other : Self) -> Self { Self :: from_unsigned (self . unsigned () . usub (other . unsigned ())) } }
};
}
