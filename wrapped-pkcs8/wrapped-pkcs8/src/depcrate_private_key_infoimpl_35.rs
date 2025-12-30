// Generated macro for impl_35 (impl)
macro_rules! Depcrate_private_key_infoimpl_35 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_35"}
// Dependencies: {}
impl < Params , Key , PubKey > fmt :: Debug for PrivateKeyInfo < Params , Key , PubKey > where Params : fmt :: Debug , PubKey : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PrivateKeyInfo") . field ("version" , & self . version ()) . field ("algorithm" , & self . algorithm) . field ("public_key" , & self . public_key) . finish_non_exhaustive () } }
};
}
