// Generated macro for tests (module)
macro_rules! Depcrate_ecdsatests {
() => {
// Module: crate::ecdsa
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "ecdsa"))] mod tests { use crate :: ecdsa :: { Signature , SigningKey , signature :: Signer } ; use hex_literal :: hex ; # [test] fn rfc6979 () { let x = hex ! ("00FAD06DAA62BA3B25D2FB40133DA757205DE67F5BB0018FEE8C86E1B68C7E75CAA896EB32F1F47C70855836A6D16FCC1466F6D8FBEC67DB89EC0C08B0E996B83538") ; let signer = SigningKey :: from_bytes (& x . into ()) . unwrap () ; let signature : Signature = signer . sign (b"sample") ; assert_eq ! (signature . to_bytes () . as_slice () , & hex ! ("00C328FAFCBD79DD77850370C46325D987CB525569FB63C5D3BC53950E6D4C5F174E25A1EE9017B5D450606ADD152B534931D7D4E8455CC91F9B15BF05EC36E377FA" "00617CCE7CF5064806C467F678D3B4080D6F1CC50AF26CA209417308281B68AF282623EAA63E5B5C0723D8B8C37FF0777B1A20F8CCB1DCCC43997F1EE0E44DA4A67A")) ; } mod sign { use crate :: { NistP521 , test_vectors :: ecdsa :: ECDSA_TEST_VECTORS } ; ecdsa_core :: new_signing_test ! (NistP521 , ECDSA_TEST_VECTORS) ; } mod verify { use crate :: { NistP521 , test_vectors :: ecdsa :: ECDSA_TEST_VECTORS } ; ecdsa_core :: new_verification_test ! (NistP521 , ECDSA_TEST_VECTORS) ; } mod wycheproof { use crate :: NistP521 ; ecdsa_core :: new_wycheproof_test ! (wycheproof , "wycheproof" , NistP521) ; } }
};
}
