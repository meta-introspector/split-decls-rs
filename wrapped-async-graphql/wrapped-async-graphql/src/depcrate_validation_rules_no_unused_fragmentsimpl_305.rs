// Generated macro for impl_305 (impl)
macro_rules! Depcrate_validation_rules_no_unused_fragmentsimpl_305 {
() => {
// Module: crate::validation::rules::no_unused_fragments
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'a > NoUnusedFragments < 'a > { fn find_reachable_fragments (& self , from : & Scope < 'a > , result : & mut HashSet < & 'a str >) { if let Scope :: Fragment (name) = * from { if result . contains (name) { return ; } else { result . insert (name) ; } } if let Some (spreads) = self . spreads . get (from) { for spread in spreads { self . find_reachable_fragments (& Scope :: Fragment (spread) , result) } } } }
};
}
