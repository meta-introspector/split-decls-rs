// Generated macro for impl_131 (impl)
macro_rules! Depcrate_arch_all_twowayimpl_131 {
() => {
// Module: crate::arch::all::twoway
// Provides: {"impl_131"}
// Dependencies: {}
impl SuffixKind { # [doc = " Returns true if and only if the given candidate byte indicates that"] # [doc = " it should replace the current suffix as the maximal (or minimal)"] # [doc = " suffix."] fn cmp (self , current : u8 , candidate : u8) -> SuffixOrdering { use self :: SuffixOrdering :: * ; match self { SuffixKind :: Minimal if candidate < current => Accept , SuffixKind :: Minimal if candidate > current => Skip , SuffixKind :: Minimal => Push , SuffixKind :: Maximal if candidate > current => Accept , SuffixKind :: Maximal if candidate < current => Skip , SuffixKind :: Maximal => Push , } } }
};
}
