// Generated macro for impl_93 (impl)
macro_rules! Depcrate_hirimpl_93 {
() => {
// Module: crate::hir
// Provides: {"impl_93"}
// Dependencies: {}
impl HirKind { # [doc = " Returns a slice of this kind's sub-expressions, if any."] fn subs (& self) -> & [Hir] { use core :: slice :: from_ref ; match * self { HirKind :: Empty | HirKind :: Char (_) | HirKind :: Class (_) | HirKind :: Look (_) => & [] , HirKind :: Repetition (Repetition { ref sub , .. }) => from_ref (sub) , HirKind :: Capture (Capture { ref sub , .. }) => from_ref (sub) , HirKind :: Concat (ref subs) => subs , HirKind :: Alternation (ref subs) => subs , } } }
};
}
