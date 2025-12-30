// Generated macro for impl_39 (impl)
macro_rules! Depcrate_private_key_infoimpl_39 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (feature = "subtle")] impl < Params , Key , PubKey > ConstantTimeEq for PrivateKeyInfo < Params , Key , PubKey > where Params : Eq , Key : PartialEq + AsRef < [u8] > , PubKey : PartialEq , { fn ct_eq (& self , other : & Self) -> Choice { let public_fields_eq = self . algorithm == other . algorithm && self . public_key == other . public_key ; self . private_key . as_ref () . ct_eq (other . private_key . as_ref ()) & Choice :: from (public_fields_eq as u8) } }
};
}
