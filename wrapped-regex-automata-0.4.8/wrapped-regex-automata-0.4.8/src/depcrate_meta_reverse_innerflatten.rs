// Generated macro for flatten (function)
macro_rules! Depcrate_meta_reverse_innerflatten {
() => {
// Module: crate::meta::reverse_inner
// Provides: {"flatten"}
// Dependencies: {}
# [doc = " Returns a copy of the given HIR but with all capturing groups removed."] fn flatten (hir : & Hir) -> Hir { match hir . kind () { HirKind :: Empty => Hir :: empty () , HirKind :: Literal (hir :: Literal (ref x)) => Hir :: literal (x . clone ()) , HirKind :: Class (ref x) => Hir :: class (x . clone ()) , HirKind :: Look (ref x) => Hir :: look (x . clone ()) , HirKind :: Repetition (ref x) => Hir :: repetition (x . with (flatten (& x . sub))) , HirKind :: Capture (hir :: Capture { ref sub , .. }) => flatten (sub) , HirKind :: Alternation (ref xs) => { Hir :: alternation (xs . iter () . map (| x | flatten (x)) . collect ()) } HirKind :: Concat (ref xs) => { Hir :: concat (xs . iter () . map (| x | flatten (x)) . collect ()) } } }
};
}
