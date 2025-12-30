// Generated macro for NamingAuthority (struct)
macro_rules! Depcrate_extensionsNamingAuthority {
() => {
// Module: crate::extensions
// Provides: {"NamingAuthority"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct NamingAuthority < 'a > { pub id : Option < asn1 :: ObjectIdentifier > , pub url : Option < asn1 :: IA5String < 'a > > , pub text : Option < DisplayText < 'a > > , }
};
}
