// Generated macro for impl_28 (impl)
macro_rules! Depcrate_private_key_infoimpl_28 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_28"}
// Dependencies: {}
impl < Params , Key , PubKey > PrivateKeyInfo < Params , Key , PubKey > { # [doc = " Create a new PKCS#8 [`PrivateKeyInfo`] message."] # [doc = ""] # [doc = " This is a helper method which initializes `attributes` and `public_key`"] # [doc = " to `None`, helpful if you aren't using those."] pub fn new (algorithm : AlgorithmIdentifier < Params > , private_key : Key) -> Self { Self { algorithm , private_key , public_key : None , } } # [doc = " Get the PKCS#8 [`Version`] for this structure."] # [doc = ""] # [doc = " [`Version::V1`] if `public_key` is `None`, [`Version::V2`] if `Some`."] pub fn version (& self) -> Version { if self . public_key . is_some () { Version :: V2 } else { Version :: V1 } } }
};
}
