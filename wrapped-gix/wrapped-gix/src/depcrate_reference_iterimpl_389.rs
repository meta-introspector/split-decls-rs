// Generated macro for impl_389 (impl)
macro_rules! Depcrate_reference_iterimpl_389 {
() => {
// Module: crate::reference::iter
// Provides: {"impl_389"}
// Dependencies: {}
impl < 'r > Iterator for Iter < '_ , 'r > { type Item = Result < crate :: Reference < 'r > , Box < dyn std :: error :: Error + Send + Sync + 'static > > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| res | { res . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync + 'static >) . and_then (| mut r | { if self . peel { let repo = & self . repo ; r . peel_to_id_packed (& repo . refs , & repo . objects , self . peel_with_packed . as_ref () . map (| p | & * * * p)) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync + 'static >) . map (| _ | r) } else { Ok (r) } }) . map (| r | crate :: Reference :: from_ref (r , self . repo)) }) } }
};
}
