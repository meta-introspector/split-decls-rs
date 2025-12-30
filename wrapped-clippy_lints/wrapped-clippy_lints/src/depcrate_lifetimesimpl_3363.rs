// Generated macro for impl_3363 (impl)
macro_rules! Depcrate_lifetimesimpl_3363 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3363"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Lifetimes { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Fn { ref sig , generics , body : id , .. } = item . kind { check_fn_inner (cx , sig , Some (id) , None , generics , item . span , true , self . msrv) ; } else if let ItemKind :: Impl (impl_) = & item . kind && ! item . span . from_expansion () { report_extra_impl_lifetimes (cx , impl_) ; } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx ImplItem < '_ >) { if let ImplItemKind :: Fn (ref sig , id) = item . kind { let report_extra_lifetimes = trait_ref_of_method (cx , item . owner_id) . is_none () ; check_fn_inner (cx , sig , Some (id) , None , item . generics , item . span , report_extra_lifetimes , self . msrv ,) ; } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx TraitItem < '_ >) { if let TraitItemKind :: Fn (ref sig , ref body) = item . kind { let (body , trait_sig) = match * body { TraitFn :: Required (sig) => (None , Some (sig)) , TraitFn :: Provided (id) => (Some (id) , None) , } ; check_fn_inner (cx , sig , body , trait_sig , item . generics , item . span , true , self . msrv) ; } } }
};
}
