// Generated macro for bad_variant_count (function)
macro_rules! Depcrate_checkbad_variant_count {
() => {
// Module: crate::check
// Provides: {"bad_variant_count"}
// Dependencies: {}
# [doc = " Emit an error when encountering two or more variants in a transparent enum."] fn bad_variant_count < 'tcx > (tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < 'tcx > , sp : Span , did : DefId) { let variant_spans : Vec < _ > = adt . variants () . iter () . map (| variant | tcx . hir_span_if_local (variant . def_id) . unwrap ()) . collect () ; let (mut spans , mut many) = (Vec :: new () , None) ; if let [start @ .. , end] = & * variant_spans { spans = start . to_vec () ; many = Some (* end) ; } tcx . dcx () . emit_err (errors :: TransparentEnumVariant { span : sp , spans , many , number : adt . variants () . len () , path : tcx . def_path_str (did) , }) ; }
};
}
