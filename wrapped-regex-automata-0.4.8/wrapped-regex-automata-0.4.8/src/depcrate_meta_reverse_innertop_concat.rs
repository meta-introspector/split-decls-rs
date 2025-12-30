// Generated macro for top_concat (function)
macro_rules! Depcrate_meta_reverse_innertop_concat {
() => {
// Module: crate::meta::reverse_inner
// Provides: {"top_concat"}
// Dependencies: {}
# [doc = " Looks for a \"top level\" HirKind::Concat item in the given HIR. This will"] # [doc = " try to return one even if it's embedded in a capturing group, but is"] # [doc = " otherwise pretty conservative in what is returned."] # [doc = ""] # [doc = " The HIR returned is a complete copy of the concat with all capturing"] # [doc = " groups removed. In effect, the concat returned is \"flattened\" with respect"] # [doc = " to capturing groups. This makes the detection logic above for prefixes"] # [doc = " a bit simpler, and it works because 1) capturing groups never influence"] # [doc = " whether a match occurs or not and 2) capturing groups are not used when"] # [doc = " doing the reverse inner search to find the start of the match."] fn top_concat (mut hir : & Hir) -> Option < Vec < Hir > > { loop { hir = match hir . kind () { HirKind :: Empty | HirKind :: Literal (_) | HirKind :: Class (_) | HirKind :: Look (_) | HirKind :: Repetition (_) | HirKind :: Alternation (_) => return None , HirKind :: Capture (hir :: Capture { ref sub , .. }) => sub , HirKind :: Concat (ref subs) => { let concat = Hir :: concat (subs . iter () . map (| h | flatten (h)) . collect ()) ; return match concat . into_kind () { HirKind :: Concat (xs) => Some (xs) , _ => return None , } ; } } ; } }
};
}
