// Generated macro for impl_218 (impl)
macro_rules! Depcrate_cpuimpl_218 {
() => {
// Module: crate::cpu
// Provides: {"impl_218"}
// Dependencies: {}
impl < A , B , C > GetFeature < (A , B , C) > for features :: Values where features :: Values : GetFeature < A > , features :: Values : GetFeature < B > , features :: Values : GetFeature < C > , { # [inline (always)] fn get_feature (& self) -> Option < (A , B , C) > { match (self . get_feature () , self . get_feature () , self . get_feature ()) { (Some (a) , Some (b) , Some (c)) => Some ((a , b , c)) , _ => None , } } }
};
}
