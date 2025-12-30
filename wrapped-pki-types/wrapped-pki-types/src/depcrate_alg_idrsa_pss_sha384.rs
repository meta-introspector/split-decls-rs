// Generated macro for RSA_PSS_SHA384 (const)
macro_rules! Depcrate_alg_idRSA_PSS_SHA384 {
() => {
// Module: crate::alg_id
// Provides: {"RSA_PSS_SHA384"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `rsassaPss` with:"] # [doc = ""] # [doc = " - hashAlgorithm: sha384"] # [doc = " - maskGenAlgorithm: mgf1 with sha384"] # [doc = " - saltLength: 48"] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # rsassa-pss"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }"] # [doc = " SEQUENCE {"] # [doc = "   # hashAlgorithm:"] # [doc = "   [0] {"] # [doc = "     SEQUENCE {"] # [doc = "       # sha384"] # [doc = "       OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }"] # [doc = "       NULL {}"] # [doc = "     }"] # [doc = "   }"] # [doc = "   # maskGenAlgorithm:"] # [doc = "   [1] {"] # [doc = "     SEQUENCE {"] # [doc = "       # mgf1"] # [doc = "       OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }"] # [doc = "       SEQUENCE {"] # [doc = "         # sha384"] # [doc = "         OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }"] # [doc = "         NULL {}"] # [doc = "       }"] # [doc = "     }"] # [doc = "   }"] # [doc = "   # saltLength:"] # [doc = "   [2] {"] # [doc = "     INTEGER { 48 }"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for"] # [doc = " the meaning of the context-specific tags."] pub const RSA_PSS_SHA384 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-rsa-pss-sha384.der")) ;
};
}
