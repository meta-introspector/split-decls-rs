// Generated macro for impl_95 (impl)
macro_rules! Depcrate_common_constraintimpl_95 {
() => {
// Module: crate::common::constraint
// Provides: {"impl_95"}
// Dependencies: {}
impl Constraint { # [doc = " Iterate over the values of this constraint."] pub fn iter < 'a > (& 'a self) -> impl Iterator < Item = i64 > + 'a { match self { Constraint :: Equal (i) => std :: slice :: Iter :: default () . copied () . chain (* i .. * i + 1) , Constraint :: Range (range) => std :: slice :: Iter :: default () . copied () . chain (range . clone ()) , Constraint :: Set (items) => items . iter () . copied () . chain (std :: ops :: Range :: default ()) , } } }
};
}
