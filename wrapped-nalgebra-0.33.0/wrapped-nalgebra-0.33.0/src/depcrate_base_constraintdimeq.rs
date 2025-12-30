// Generated macro for DimEq (trait)
macro_rules! Depcrate_base_constraintDimEq {
() => {
// Module: crate::base::constraint
// Provides: {"DimEq"}
// Dependencies: {}
# [doc = " Constrains `D1` and `D2` to be equivalent."] pub trait DimEq < D1 : Dim , D2 : Dim > { # [doc = " This is either equal to `D1` or `D2`, always choosing the one (if any) which is a type-level"] # [doc = " constant."] type Representative : Dim ; # [doc = " This constructs a value of type `Representative` with the"] # [doc = " correct value"] fn representative (d1 : D1 , d2 : D2) -> Option < Self :: Representative > { if d1 . value () != d2 . value () { None } else { Some (Self :: Representative :: from_usize (d1 . value ())) } } }
};
}
