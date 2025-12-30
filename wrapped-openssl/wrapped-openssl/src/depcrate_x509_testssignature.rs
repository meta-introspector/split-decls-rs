// Generated macro for signature (function)
macro_rules! Depcrate_x509_testssignature {
() => {
// Module: crate::x509::tests
// Provides: {"signature"}
// Dependencies: {}
# [test] fn signature () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let signature = cert . signature () ; assert_eq ! (hex :: encode (signature . as_slice ()) , "4af607b889790b43470442cfa551cdb8b6d0b0340d2958f76b9e3ef6ad4992230cead6842587f0ecad5\
         78e6e11a221521e940187e3d6652de14e84e82f6671f097cc47932e022add3c0cb54a26bf27fa84c107\
         4971caa6bee2e42d34a5b066c427f2d452038082b8073993399548088429de034fdd589dcfb0dd33be7\
         ebdfdf698a28d628a89568881d658151276bde333600969502c4e62e1d3470a683364dfb241f78d310a\
         89c119297df093eb36b7fd7540224f488806780305d1e79ffc938fe2275441726522ab36d88348e6c51\
         f13dcc46b5e1cdac23c974fd5ef86aa41e91c9311655090a52333bc79687c748d833595d4c5f987508f\
         e121997410d37c") ; let algorithm = cert . signature_algorithm () ; assert_eq ! (algorithm . object () . nid () , Nid :: SHA256WITHRSAENCRYPTION) ; assert_eq ! (algorithm . object () . to_string () , "sha256WithRSAEncryption") ; }
};
}
