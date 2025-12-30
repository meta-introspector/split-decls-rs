// Generated macro for impl_244 (impl)
macro_rules! Depcrate_hir_visitorimpl_244 {
() => {
// Module: crate::hir::visitor
// Provides: {"impl_244"}
// Dependencies: {}
impl < 'a > Frame < 'a > { # [doc = " Perform the next inductive step on this frame and return the next"] # [doc = " child HIR node to visit."] fn child (& self) -> & 'a Hir { match * self { Frame :: Repetition (rep) => & rep . sub , Frame :: Capture (capture) => & capture . sub , Frame :: Concat { head , .. } => head , Frame :: Alternation { head , .. } => head , } } }
};
}
