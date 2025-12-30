// Generated macro for test (module)
macro_rules! Depcrate_derivetest {
() => {
// Module: crate::derive
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: ec :: { EcGroup , EcKey } ; use crate :: nid :: Nid ; use crate :: pkey :: PKey ; # [test] fn derive_without_peer () { let group = EcGroup :: from_curve_name (Nid :: X9_62_PRIME256V1) . unwrap () ; let ec_key = EcKey :: generate (& group) . unwrap () ; let pkey = PKey :: from_ec_key (ec_key) . unwrap () ; let mut deriver = Deriver :: new (& pkey) . unwrap () ; deriver . derive_to_vec () . unwrap_err () ; } # [test] fn test_ec_key_derive () { let group = EcGroup :: from_curve_name (Nid :: X9_62_PRIME256V1) . unwrap () ; let ec_key = EcKey :: generate (& group) . unwrap () ; let ec_key2 = EcKey :: generate (& group) . unwrap () ; let pkey = PKey :: from_ec_key (ec_key) . unwrap () ; let pkey2 = PKey :: from_ec_key (ec_key2) . unwrap () ; let mut deriver = Deriver :: new (& pkey) . unwrap () ; deriver . set_peer (& pkey2) . unwrap () ; let shared = deriver . derive_to_vec () . unwrap () ; assert ! (! shared . is_empty ()) ; } # [test] # [cfg (ossl300)] fn test_ec_key_derive_ex () { let group = EcGroup :: from_curve_name (Nid :: X9_62_PRIME256V1) . unwrap () ; let ec_key = EcKey :: generate (& group) . unwrap () ; let ec_key2 = EcKey :: generate (& group) . unwrap () ; let pkey = PKey :: from_ec_key (ec_key) . unwrap () ; let pkey2 = PKey :: from_ec_key (ec_key2) . unwrap () ; let mut deriver = Deriver :: new (& pkey) . unwrap () ; deriver . set_peer_ex (& pkey2 , true) . unwrap () ; let shared = deriver . derive_to_vec () . unwrap () ; assert ! (! shared . is_empty ()) ; } }
};
}
