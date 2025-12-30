// Generated macro for signature_help_for_record_lit (function)
macro_rules! Depcrate_signature_helpsignature_help_for_record_lit {
() => {
// Module: crate::signature_help
// Provides: {"signature_help_for_record_lit"}
// Dependencies: {}
fn signature_help_for_record_lit (sema : & Semantics < '_ , RootDatabase > , record : ast :: RecordExpr , token : SyntaxToken , edition : Edition , display_target : DisplayTarget ,) -> Option < SignatureHelp > { signature_help_for_record_ (sema , record . record_expr_field_list () ? . syntax () . children_with_tokens () , & record . path () ? , record . record_expr_field_list () ? . fields () . filter_map (| field | sema . resolve_record_field (& field)) . map (| (field , _ , ty) | (field , ty)) , token , edition , display_target ,) }
};
}
