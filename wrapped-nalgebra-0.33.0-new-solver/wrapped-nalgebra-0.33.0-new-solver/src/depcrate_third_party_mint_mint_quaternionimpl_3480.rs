// Generated macro for impl_3480 (impl)
macro_rules! Depcrate_third_party_mint_mint_quaternionimpl_3480 {
() => {
// Module: crate::third_party::mint::mint_quaternion
// Provides: {"impl_3480"}
// Dependencies: {}
impl < T : Scalar > From < mint :: Quaternion < T > > for Quaternion < T > { fn from (q : mint :: Quaternion < T >) -> Self { Self :: new (q . s , q . v . x , q . v . y , q . v . z) } }
};
}
