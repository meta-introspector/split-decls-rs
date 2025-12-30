// Generated macro for impl_149 (impl)
macro_rules! Depcrate_definitionsimpl_149 {
() => {
// Module: crate::definitions
// Provides: {"impl_149"}
// Dependencies: {}
impl DefKey { pub (crate) fn compute_stable_hash (& self , parent : DefPathHash) -> DefPathHash { let mut hasher = StableHasher :: new () ; parent . local_hash () . hash (& mut hasher) ; let DisambiguatedDefPathData { ref data , disambiguator } = self . disambiguated_data ; std :: mem :: discriminant (data) . hash (& mut hasher) ; if let Some (name) = data . hashed_symbol () { name . as_str () . hash (& mut hasher) ; } disambiguator . hash (& mut hasher) ; let local_hash = hasher . finish () ; DefPathHash :: new (parent . stable_crate_id () , local_hash) } # [inline] pub fn get_opt_name (& self) -> Option < Symbol > { self . disambiguated_data . data . get_opt_name () } }
};
}
