// Generated macro for impl_3481 (impl)
macro_rules! Depcrate_third_party_mint_mint_quaternionimpl_3481 {
() => {
// Module: crate::third_party::mint::mint_quaternion
// Provides: {"impl_3481"}
// Dependencies: {}
impl < T : Scalar > Into < mint :: Quaternion < T > > for Quaternion < T > { fn into (self) -> mint :: Quaternion < T > { mint :: Quaternion { v : mint :: Vector3 { x : self [0] . clone () , y : self [1] . clone () , z : self [2] . clone () , } , s : self [3] . clone () , } } }
};
}
