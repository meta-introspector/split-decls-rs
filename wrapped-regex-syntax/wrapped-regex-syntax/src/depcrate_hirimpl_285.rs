// Generated macro for impl_285 (impl)
macro_rules! Depcrate_hirimpl_285 {
() => {
// Module: crate::hir
// Provides: {"impl_285"}
// Dependencies: {}
# [doc = " A custom `Drop` impl is used for `HirKind` such that it uses constant stack"] # [doc = " space but heap space proportional to the depth of the total `Hir`."] impl Drop for Hir { fn drop (& mut self) { use core :: mem ; match * self . kind () { HirKind :: Empty | HirKind :: Literal (_) | HirKind :: Class (_) | HirKind :: Look (_) => return , HirKind :: Capture (ref x) if x . sub . kind . subs () . is_empty () => return , HirKind :: Repetition (ref x) if x . sub . kind . subs () . is_empty () => { return } HirKind :: Concat (ref x) if x . is_empty () => return , HirKind :: Alternation (ref x) if x . is_empty () => return , _ => { } } let mut stack = vec ! [mem :: replace (self , Hir :: empty ())] ; while let Some (mut expr) = stack . pop () { match expr . kind { HirKind :: Empty | HirKind :: Literal (_) | HirKind :: Class (_) | HirKind :: Look (_) => { } HirKind :: Capture (ref mut x) => { stack . push (mem :: replace (& mut x . sub , Hir :: empty ())) ; } HirKind :: Repetition (ref mut x) => { stack . push (mem :: replace (& mut x . sub , Hir :: empty ())) ; } HirKind :: Concat (ref mut x) => { stack . extend (x . drain (..)) ; } HirKind :: Alternation (ref mut x) => { stack . extend (x . drain (..)) ; } } } } }
};
}
