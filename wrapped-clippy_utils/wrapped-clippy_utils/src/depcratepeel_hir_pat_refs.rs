// Generated macro for peel_hir_pat_refs (function)
macro_rules! Depcratepeel_hir_pat_refs {
() => {
// Module: crate
// Provides: {"peel_hir_pat_refs"}
// Dependencies: {}
# [doc = " Peels off all references on the pattern. Returns the underlying pattern and the number of"] # [doc = " references removed."] pub fn peel_hir_pat_refs < 'a > (pat : & 'a Pat < 'a >) -> (& 'a Pat < 'a > , usize) { fn peel < 'a > (pat : & 'a Pat < 'a > , count : usize) -> (& 'a Pat < 'a > , usize) { if let PatKind :: Ref (pat , _ , _) = pat . kind { peel (pat , count + 1) } else { (pat , count) } } peel (pat , 0) }
};
}
