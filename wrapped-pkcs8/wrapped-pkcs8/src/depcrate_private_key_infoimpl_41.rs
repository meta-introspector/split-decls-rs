// Generated macro for impl_41 (impl)
macro_rules! Depcrate_private_key_infoimpl_41 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "subtle")] impl < Params , Key , PubKey > PartialEq for PrivateKeyInfo < Params , Key , PubKey > where Params : Eq , Key : PartialEq + AsRef < [u8] > , PubKey : PartialEq , { fn eq (& self , other : & Self) -> bool { self . ct_eq (other) . into () } }
};
}
