// Generated macro for impl_3605 (impl)
macro_rules! Depcrate_pg_valueimpl_3605 {
() => {
// Module: crate::pg::value
// Provides: {"impl_3605"}
// Dependencies: {}
impl < 'a > PgValue < 'a > { # [cfg (test)] pub (crate) fn for_test (raw_value : & 'a [u8]) -> Self { # [allow (unsafe_code)] static FAKE_OID : NonZeroU32 = unsafe { NonZeroU32 :: new_unchecked (42) } ; Self { raw_value , type_oid_lookup : & FAKE_OID , } } # [doc = " Create a new instance of `PgValue` based on a byte buffer"] # [doc = " and a way to receive information about the type of the value"] # [doc = " represented by the buffer"] # [cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub fn new (raw_value : & 'a [u8] , type_oid_lookup : & 'a dyn TypeOidLookup) -> Self { Self :: new_internal (raw_value , type_oid_lookup) } pub (in crate :: pg) fn new_internal (raw_value : & 'a [u8] , type_oid_lookup : & 'a dyn TypeOidLookup ,) -> Self { Self { raw_value , type_oid_lookup , } } # [doc = " Get the underlying raw byte representation"] pub fn as_bytes (& self) -> & [u8] { self . raw_value } # [doc = " Get the type oid of this value"] pub fn get_oid (& self) -> NonZeroU32 { self . type_oid_lookup . lookup () } pub (crate) fn subslice (& self , range : Range < usize >) -> Self { Self { raw_value : & self . raw_value [range] , .. * self } } }
};
}
