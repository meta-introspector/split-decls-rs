// Generated macro for impl_26 (impl)
macro_rules! Depcrate_interruptimpl_26 {
() => {
// Module: crate::interrupt
// Provides: {"impl_26"}
// Dependencies: {}
impl < I , EFN , E > Iterator for IterWithErr < '_ , I , EFN > where I : Iterator , EFN : FnOnce () -> E , { type Item = Result < I :: Item , E > ; fn next (& mut self) -> Option < Self :: Item > { self . make_err . as_ref () ? ; if self . should_interrupt . load (Ordering :: Relaxed) { return self . make_err . take () . map (| f | Err (f ())) ; } match self . inner . next () { Some (next) => Some (Ok (next)) , None => { self . make_err = None ; None } } } }
};
}
