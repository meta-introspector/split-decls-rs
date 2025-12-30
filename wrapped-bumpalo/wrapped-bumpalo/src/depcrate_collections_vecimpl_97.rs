// Generated macro for impl_97 (impl)
macro_rules! Depcrate_collections_vecimpl_97 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'bump , T : 'bump + Clone > Clone for Vec < 'bump , T > { # [cfg (not (test))] fn clone (& self) -> Vec < 'bump , T > { let mut v = Vec :: with_capacity_in (self . len () , self . buf . bump ()) ; v . extend (self . iter () . cloned ()) ; v } # [cfg (test)] fn clone (& self) -> Vec < 'bump , T > { let mut v = Vec :: new_in (self . buf . bump ()) ; v . extend (self . iter () . cloned ()) ; v } }
};
}
