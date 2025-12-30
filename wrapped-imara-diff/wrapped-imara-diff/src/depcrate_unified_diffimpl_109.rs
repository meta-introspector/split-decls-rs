// Generated macro for impl_109 (impl)
macro_rules! Depcrate_unified_diffimpl_109 {
() => {
// Module: crate::unified_diff
// Provides: {"impl_109"}
// Dependencies: {}
impl Diff { pub fn unified_diff < 'a , P : UnifiedDiffPrinter , T : Hash + Eq > (& 'a self , printer : & 'a P , config : UnifiedDiffConfig , input : & 'a InternedInput < T > ,) -> UnifiedDiff < 'a , P > { self . unified_diff_with (printer , config , & input . before , & input . after) } pub fn unified_diff_with < 'a , P : UnifiedDiffPrinter > (& 'a self , printer : & 'a P , config : UnifiedDiffConfig , before : & 'a [Token] , after : & 'a [Token] ,) -> UnifiedDiff < 'a , P > { UnifiedDiff { printer , diff : self , config , before , after , } } }
};
}
