// Generated macro for strip_pat_refs (function)
macro_rules! Depcratestrip_pat_refs {
() => {
// Module: crate
// Provides: {"strip_pat_refs"}
// Dependencies: {}
pub fn strip_pat_refs < 'hir > (mut pat : & 'hir Pat < 'hir >) -> & 'hir Pat < 'hir > { while let PatKind :: Ref (subpat , _ , _) = pat . kind { pat = subpat ; } pat }
};
}
