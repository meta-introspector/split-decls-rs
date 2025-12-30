// Generated macro for impl_99 (impl)
macro_rules! Depcrate_targetimpl_99 {
() => {
// Module: crate::target
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > From < TargetRef < 'a > > for Target { fn from (src : TargetRef < 'a >) -> Self { match src { TargetRef :: Object (oid) => Target :: Object (oid . to_owned ()) , TargetRef :: Symbolic (name) => Target :: Symbolic (name . to_owned ()) , } } }
};
}
