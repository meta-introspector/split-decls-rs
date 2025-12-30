// Generated macro for impl_217 (impl)
macro_rules! Depcrate_cpuimpl_217 {
() => {
// Module: crate::cpu
// Provides: {"impl_217"}
// Dependencies: {}
impl < A , B > GetFeature < (A , B) > for features :: Values where features :: Values : GetFeature < A > , features :: Values : GetFeature < B > , { # [inline (always)] fn get_feature (& self) -> Option < (A , B) > { match (self . get_feature () , self . get_feature ()) { (Some (a) , Some (b)) => Some ((a , b)) , _ => None , } } }
};
}
