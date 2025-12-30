// Generated macro for impl_486 (impl)
macro_rules! Depcrate_repository_revision_explainimpl_486 {
() => {
// Module: crate::repository::revision::explain
// Provides: {"impl_486"}
// Dependencies: {}
impl delegate :: Kind for Explain < '_ > { fn kind (& mut self , kind : spec :: Kind) -> Option < () > { self . prefix () ? ; self . call = 0 ; writeln ! (self . out , "Set revision specification to {} mode" , match kind { spec :: Kind :: RangeBetween => "range" , spec :: Kind :: ReachableToMergeBase => "merge-base" , spec :: Kind :: ExcludeReachable => "exclude" , spec :: Kind :: IncludeReachableFromParents => "include parents" , spec :: Kind :: ExcludeReachableFromParents => "exclude parents" , spec :: Kind :: IncludeReachable => unreachable ! ("BUG: 'single' mode is implied but cannot be set explicitly") , }) . ok () } }
};
}
