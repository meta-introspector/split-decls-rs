// Generated macro for tests (module)
macro_rules! Depcrate_ecdsatests {
() => {
// Module: crate::ecdsa
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "ecdsa"))] mod tests { use crate :: ecdsa :: { Signature , SigningKey , signature :: Signer } ; use hex_literal :: hex ; # [test] fn rfc6979 () { let x = hex ! ("F220266E1105BFE3083E03EC7A3A654651F45E37167E88600BF257C1") ; let signer = SigningKey :: from_bytes (& x . into ()) . unwrap () ; let signature : Signature = signer . sign (b"sample") ; assert_eq ! (signature . to_bytes () . as_slice () , & hex ! ("1CDFE6662DDE1E4A1EC4CDEDF6A1F5A2FB7FBD9145C12113E6ABFD3E
                 A6694FD7718A21053F225D3F46197CA699D45006C06F871808F43EBC")) ; let signature : Signature = signer . sign (b"test") ; assert_eq ! (signature . to_bytes () . as_slice () , & hex ! ("C441CE8E261DED634E4CF84910E4C5D1D22C5CF3B732BB204DBEF019
                 902F42847A63BDC5F6046ADA114953120F99442D76510150F372A3F4")) ; } mod sign { use crate :: { NistP224 , test_vectors :: ecdsa :: ECDSA_TEST_VECTORS } ; ecdsa_core :: new_signing_test ! (NistP224 , ECDSA_TEST_VECTORS) ; } mod verify { use crate :: { NistP224 , test_vectors :: ecdsa :: ECDSA_TEST_VECTORS } ; ecdsa_core :: new_verification_test ! (NistP224 , ECDSA_TEST_VECTORS) ; } mod wycheproof { use crate :: NistP224 ; ecdsa_core :: new_wycheproof_test ! (wycheproof , "wycheproof" , NistP224) ; } }
};
}
