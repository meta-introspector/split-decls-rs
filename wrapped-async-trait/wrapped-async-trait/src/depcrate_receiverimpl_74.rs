// Generated macro for impl_74 (impl)
macro_rules! Depcrate_receiverimpl_74 {
() => {
// Module: crate::receiver
// Provides: {"impl_74"}
// Dependencies: {}
impl VisitMut for HasMutPat { fn visit_pat_ident_mut (& mut self , i : & mut PatIdent) { if let Some (m) = & i . mutability { self . 0 = Some (Token ! [mut] (m . span)) ; } else { visit_mut :: visit_pat_ident_mut (self , i) ; } } }
};
}
