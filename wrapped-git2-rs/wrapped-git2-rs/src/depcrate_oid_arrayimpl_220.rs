// Generated macro for impl_220 (impl)
macro_rules! Depcrate_oid_arrayimpl_220 {
() => {
// Module: crate::oid_array
// Provides: {"impl_220"}
// Dependencies: {}
impl Deref for OidArray { type Target = [Oid] ; fn deref (& self) -> & [Oid] { unsafe { debug_assert_eq ! (mem :: size_of ::< Oid > () , mem :: size_of_val (&* self . raw . ids)) ; slice :: from_raw_parts (self . raw . ids as * const Oid , self . raw . count as usize) } } }
};
}
