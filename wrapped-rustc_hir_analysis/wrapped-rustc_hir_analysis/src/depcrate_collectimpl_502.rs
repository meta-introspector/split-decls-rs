// Generated macro for impl_502 (impl)
macro_rules! Depcrate_collectimpl_502 {
() => {
// Module: crate::collect
// Provides: {"impl_502"}
// Dependencies: {}
impl < 'tcx > ItemCtxt < 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , item_def_id : LocalDefId) -> ItemCtxt < 'tcx > { ItemCtxt { tcx , item_def_id , tainted_by_errors : Cell :: new (None) } } pub (crate) fn lower_ty (& self , hir_ty : & hir :: Ty < 'tcx >) -> Ty < 'tcx > { self . lowerer () . lower_ty (hir_ty) } pub (crate) fn hir_id (& self) -> hir :: HirId { self . tcx . local_def_id_to_hir_id (self . item_def_id) } pub (crate) fn node (& self) -> hir :: Node < 'tcx > { self . tcx . hir_node (self . hir_id ()) } fn check_tainted_by_errors (& self) -> Result < () , ErrorGuaranteed > { match self . tainted_by_errors . get () { Some (err) => Err (err) , None => Ok (()) , } } fn report_placeholder_type_error (& self , placeholder_types : Vec < Span > , infer_replacements : Vec < (Span , String) > ,) -> ErrorGuaranteed { let node = self . tcx . hir_node_by_def_id (self . item_def_id) ; let generics = node . generics () ; let kind_id = match node { Node :: GenericParam (_) | Node :: WherePredicate (_) | Node :: Field (_) => { self . tcx . local_parent (self . item_def_id) } _ => self . item_def_id , } ; let kind = self . tcx . def_descr (kind_id . into ()) ; let mut diag = placeholder_type_error_diag (self , generics , placeholder_types , infer_replacements . iter () . map (| & (span , _) | span) . collect () , false , None , kind ,) ; if ! infer_replacements . is_empty () { diag . multipart_suggestion (format ! ("try replacing `_` with the type{} in the corresponding trait method \
                        signature" , rustc_errors :: pluralize ! (infer_replacements . len ()) ,) , infer_replacements , Applicability :: MachineApplicable ,) ; } diag . emit () } }
};
}
