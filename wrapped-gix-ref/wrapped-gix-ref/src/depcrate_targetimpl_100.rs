// Generated macro for impl_100 (impl)
macro_rules! Depcrate_targetimpl_100 {
() => {
// Module: crate::target
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a > PartialEq < TargetRef < 'a > > for Target { fn eq (& self , other : & TargetRef < 'a >) -> bool { match (self , other) { (Target :: Object (lhs) , TargetRef :: Object (rhs)) => lhs == rhs , (Target :: Symbolic (lhs) , TargetRef :: Symbolic (rhs)) => lhs . as_bstr () == rhs . as_bstr () , _ => false , } } }
};
}
