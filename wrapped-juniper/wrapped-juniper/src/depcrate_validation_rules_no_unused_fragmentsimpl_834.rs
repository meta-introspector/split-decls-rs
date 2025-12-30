// Generated macro for impl_834 (impl)
macro_rules! Depcrate_validation_rules_no_unused_fragmentsimpl_834 {
() => {
// Module: crate::validation::rules::no_unused_fragments
// Provides: {"impl_834"}
// Dependencies: {}
impl < 'a > NoUnusedFragments < 'a > { fn find_reachable_fragments (& 'a self , from : Scope < 'a > , result : & mut HashSet < & 'a str >) { let mut to_visit = Vec :: new () ; to_visit . push (from) ; while let Some (from) = to_visit . pop () { if let Some (next) = self . find_reachable_fragments_inner (from , result) { to_visit . extend (next . iter () . map (| s | Scope :: Fragment (s))) ; } } } # [doc = " This function should be called only inside"] # [doc = " [`Self::find_reachable_fragments()`], as it's a recursive function using"] # [doc = " heap instead of a stack. So, instead of the recursive call, we return a"] # [doc = " [`Vec`] that is visited inside [`Self::find_reachable_fragments()`]."] fn find_reachable_fragments_inner (& 'a self , from : Scope < 'a > , result : & mut HashSet < & 'a str > ,) -> Option < & 'a Vec < & 'a str > > { if let Scope :: Fragment (name) = from { if result . contains (name) { return None ; } else { result . insert (name) ; } } self . spreads . get (& from) } }
};
}
