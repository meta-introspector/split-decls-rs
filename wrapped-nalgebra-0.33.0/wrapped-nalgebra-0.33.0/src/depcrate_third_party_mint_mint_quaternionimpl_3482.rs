// Generated macro for impl_3482 (impl)
macro_rules! Depcrate_third_party_mint_mint_quaternionimpl_3482 {
() => {
// Module: crate::third_party::mint::mint_quaternion
// Provides: {"impl_3482"}
// Dependencies: {}
impl < T : Scalar + SimdValue > Into < mint :: Quaternion < T > > for UnitQuaternion < T > { fn into (self) -> mint :: Quaternion < T > { mint :: Quaternion { v : mint :: Vector3 { x : self [0] . clone () , y : self [1] . clone () , z : self [2] . clone () , } , s : self [3] . clone () , } } }
};
}
