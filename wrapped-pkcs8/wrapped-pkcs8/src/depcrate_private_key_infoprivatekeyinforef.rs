// Generated macro for PrivateKeyInfoRef (type)
macro_rules! Depcrate_private_key_infoPrivateKeyInfoRef {
() => {
// Module: crate::private_key_info
// Provides: {"PrivateKeyInfoRef"}
// Dependencies: {}
# [doc = " [`PrivateKeyInfo`] with [`AnyRef`] algorithm parameters, and `&[u8]` key."] pub type PrivateKeyInfoRef < 'a > = PrivateKeyInfo < AnyRef < 'a > , & 'a OctetStringRef , BitStringRef < 'a > > ;
};
}
