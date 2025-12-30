// Generated macro for tests (module)
macro_rules! Depcrate_signaturetests {
() => {
// Module: crate::signature
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: rand :: { generate , SystemRandom } ; use crate :: signature :: { ParsedPublicKey , UnparsedPublicKey , ED25519 } ; use crate :: test ; use regex :: Regex ; # [cfg (feature = "fips")] mod fips ; # [test] fn test_unparsed_public_key () { let random_pubkey : [u8 ; 32] = generate (& SystemRandom :: new ()) . unwrap () . expose () ; let unparsed_pubkey = UnparsedPublicKey :: new (& ED25519 , random_pubkey) ; let unparsed_pubkey_debug = format ! ("{:?}" , & unparsed_pubkey) ; # [allow (clippy :: clone_on_copy)] let unparsed_pubkey_clone = unparsed_pubkey . clone () ; assert_eq ! (unparsed_pubkey_debug , format ! ("{unparsed_pubkey_clone:?}")) ; let pubkey_re = Regex :: new ("UnparsedPublicKey \\{ algorithm: EdDSAParameters, bytes: \"[0-9a-f]{64}\" \\}" ,) . unwrap () ; assert ! (pubkey_re . is_match (& unparsed_pubkey_debug)) ; } # [test] fn test_types () { test :: compile_time_assert_send :: < UnparsedPublicKey < & [u8] > > () ; test :: compile_time_assert_sync :: < UnparsedPublicKey < & [u8] > > () ; test :: compile_time_assert_send :: < UnparsedPublicKey < Vec < u8 > > > () ; test :: compile_time_assert_sync :: < UnparsedPublicKey < Vec < u8 > > > () ; test :: compile_time_assert_clone :: < UnparsedPublicKey < & [u8] > > () ; test :: compile_time_assert_clone :: < UnparsedPublicKey < Vec < u8 > > > () ; test :: compile_time_assert_send :: < ParsedPublicKey > () ; test :: compile_time_assert_sync :: < ParsedPublicKey > () ; test :: compile_time_assert_clone :: < ParsedPublicKey > () ; } }
};
}
