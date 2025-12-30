// Generated macro for extract_ctor_call (function)
macro_rules! Depcrate_question_markextract_ctor_call {
() => {
// Module: crate::question_mark
// Provides: {"extract_ctor_call"}
// Dependencies: {}
fn extract_ctor_call < 'a , 'tcx > (cx : & LateContext < 'tcx > , expected_ctor : LangItem , pat : & 'a Pat < 'tcx > ,) -> Option < & 'a Pat < 'tcx > > { if let PatKind :: TupleStruct (variant_path , [val_binding] , _) = & pat . kind && cx . qpath_res (variant_path , pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , expected_ctor) { Some (val_binding) } else { None } }
};
}
