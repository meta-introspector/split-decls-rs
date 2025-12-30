// Generated macro for Challenge (struct)
macro_rules! Depcrate_popChallenge {
() => {
// Module: crate::pop
// Provides: {"Challenge"}
// Dependencies: {}
# [doc = " The `Challenge` type is defined in [RFC 4210 Section 5.2.8.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "  Challenge ::= SEQUENCE {"] # [doc = "      owf                 AlgorithmIdentifier{DIGEST-ALGORITHM, {...}}"] # [doc = "                              OPTIONAL,"] # [doc = "      witness             OCTET STRING,"] # [doc = "      challenge           OCTET STRING"] # [doc = "      -- the encryption (under the public key for which the cert."] # [doc = "      -- request is being made) of Rand, where Rand is specified as"] # [doc = "      --   Rand ::= SEQUENCE {"] # [doc = "      --      int      INTEGER,"] # [doc = "      --       - the randomly-generated INTEGER A (above)"] # [doc = "      --      sender   GeneralName"] # [doc = "      --       - the sender's name (as included in PKIHeader)"] # [doc = "      --   }"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.8.3]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.8.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct Challenge { pub owf : Option < AlgorithmIdentifierOwned > , pub witness : OctetString , pub challenge : OctetString , }
};
}
