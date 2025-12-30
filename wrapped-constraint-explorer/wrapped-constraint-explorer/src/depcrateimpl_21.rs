// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl From < Constraint > for ConstraintName { fn from (constraint : Constraint) -> Self { match constraint { Length (_) => Self :: Length , Percentage (_) => Self :: Percentage , Ratio (_ , _) => Self :: Ratio , Min (_) => Self :: Min , Max (_) => Self :: Max , Fill (_) => Self :: Fill , } } }
};
}
