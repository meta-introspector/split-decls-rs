// Generated macro for DynAssociatedOid (trait)
macro_rules! Depcrate_traitsDynAssociatedOid {
() => {
// Module: crate::traits
// Provides: {"DynAssociatedOid"}
// Dependencies: {}
# [doc = " A trait which associates a dynamic, `&self`-dependent OID with a type,"] # [doc = " which may change depending on the type's value."] # [doc = ""] # [doc = " This trait is object safe and auto-impl'd for any types which impl"] # [doc = " [`AssociatedOid`]."] pub trait DynAssociatedOid { # [doc = " Get the OID associated with this value."] fn oid (& self) -> ObjectIdentifier ; }
};
}
