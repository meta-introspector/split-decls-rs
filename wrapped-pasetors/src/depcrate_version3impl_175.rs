// Generated macro for impl_175 (impl)
macro_rules! Depcrate_version3impl_175 {
() => {
// Module: crate::version3
// Provides: {"impl_175"}
// Dependencies: {}
impl TryFrom < & UncompressedPublicKey > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & UncompressedPublicKey) -> Result < Self , Self :: Error > { Ok (Self { bytes : value . 0 . to_encoded_point (true) . as_ref () . to_vec () , phantom : PhantomData , }) } }
};
}
