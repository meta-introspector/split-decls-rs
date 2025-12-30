// Generated macro for oids (module)
macro_rules! Depcrateoids {
() => {
// Module: crate
// Provides: {"oids"}
// Dependencies: {}
# [cfg (feature = "oid")] mod oids { use digest :: const_oid :: { AssociatedOid , ObjectIdentifier } ; impl AssociatedOid for super :: Gost94CryptoPro { const OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("1.2.643.2.2.9") ; } impl AssociatedOid for super :: Gost94UA { const OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("1.2.804.2.1.1.1.1.2.1") ; } }
};
}
