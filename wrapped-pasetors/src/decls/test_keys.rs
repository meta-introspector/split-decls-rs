macro_rules! deps {
    () => {
        AsymmetricKeyPair!();
        AsymmetricSecretKey!();
        AsymmetricPublicKey!();
        V4!();
        SymmetricKey!();
    };
}

macro_rules! test_keys {
    () => {
        deps!();
        # [cfg (test)] mod test_keys { use super :: * ; use crate :: version4 :: test_tokens :: TEST_SK_BYTES ; # [test] fn test_symmetric_gen () { let randomv = SymmetricKey :: < V4 > :: generate () . unwrap () ; assert_ne ! (randomv . as_bytes () , & [0u8 ; 32]) ; } # [test] fn test_invalid_sizes () { assert ! (AsymmetricSecretKey ::< V4 >:: from (& [1u8 ; 63]) . is_err ()) ; assert ! (AsymmetricSecretKey ::< V4 >:: from (& TEST_SK_BYTES) . is_ok ()) ; assert ! (AsymmetricSecretKey ::< V4 >:: from (& [1u8 ; 65]) . is_err ()) ; assert ! (AsymmetricPublicKey ::< V4 >:: from (& [1u8 ; 31]) . is_err ()) ; assert ! (AsymmetricPublicKey ::< V4 >:: from (& [1u8 ; 32]) . is_ok ()) ; assert ! (AsymmetricPublicKey ::< V4 >:: from (& [1u8 ; 33]) . is_err ()) ; assert ! (SymmetricKey ::< V4 >:: from (& [0u8 ; 31]) . is_err ()) ; assert ! (SymmetricKey ::< V4 >:: from (& [0u8 ; 32]) . is_ok ()) ; assert ! (SymmetricKey ::< V4 >:: from (& [0u8 ; 33]) . is_err ()) ; } # [test] fn try_from_secret_to_public () { let kpv4 = AsymmetricKeyPair :: < V4 > :: generate () . unwrap () ; let pubv4 = AsymmetricPublicKey :: < V4 > :: try_from (& kpv4 . secret) . unwrap () ; assert_eq ! (pubv4 . as_bytes () , kpv4 . public . as_bytes ()) ; assert_eq ! (pubv4 , kpv4 . public) ; assert_eq ! (& kpv4 . secret . as_bytes () [32 ..] , pubv4 . as_bytes ()) ; } # [test] fn test_trait_impls () { let debug = format ! ("{:?}" , SymmetricKey ::< V4 >:: generate () . unwrap ()) ; assert_eq ! (debug , "SymmetricKey {***OMITTED***}") ; let randomv = SymmetricKey :: < V4 > :: generate () . unwrap () ; let zero = SymmetricKey :: < V4 > :: from (& [0u8 ; V4 :: LOCAL_KEY]) . unwrap () ; assert_ne ! (randomv , zero) ; let debug = format ! ("{:?}" , AsymmetricKeyPair ::< V4 >:: generate () . unwrap () . secret) ; assert_eq ! (debug , "AsymmetricSecretKey {***OMITTED***}") ; let random1 = AsymmetricKeyPair :: < V4 > :: generate () . unwrap () ; let random2 = AsymmetricKeyPair :: < V4 > :: generate () . unwrap () ; assert_ne ! (random1 . secret , random2 . secret) ; } # [test] fn test_clone () { let sk = SymmetricKey :: < V4 > :: generate () . unwrap () ; assert_eq ! (sk , sk . clone ()) ; let kp = AsymmetricKeyPair :: < V4 > :: generate () . unwrap () ; assert_eq ! (kp . secret , kp . secret . clone ()) ; assert_eq ! (kp . public , kp . public . clone ()) ; } }
    };
}

test_keys!();