// Generated macro for SameDimension (trait)
macro_rules! Depcrate_base_constraintSameDimension {
() => {
// Module: crate::base::constraint
// Provides: {"SameDimension"}
// Dependencies: {}
# [doc = " Constrains D1 and D2 to be equivalent, where they both designate dimensions of algebraic"] # [doc = " entities (e.g. square matrices)."] pub trait SameDimension < D1 : Dim , D2 : Dim > : SameNumberOfRows < D1 , D2 > + SameNumberOfColumns < D1 , D2 > { # [doc = " This is either equal to `D1` or `D2`, always choosing the one (if any) which is a type-level"] # [doc = " constant."] type Representative : Dim ; }
};
}
