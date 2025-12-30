// Generated macro for impl_69 (impl)
macro_rules! Depcrate_public_keyimpl_69 {
() => {
// Module: crate::public_key
// Provides: {"impl_69"}
// Dependencies: {}
impl From < PublicKey > for EncodedPoint { fn from (value : PublicKey) -> Self { value . point . to_encoded_point (false) } }
};
}
