// Generated macro for signature_help_for_record_pat (function)
macro_rules! Depcrate_signature_helpsignature_help_for_record_pat {
() => {
// Module: crate::signature_help
// Provides: {"signature_help_for_record_pat"}
// Dependencies: {}
fn signature_help_for_record_pat (sema : & Semantics < '_ , RootDatabase > , record : ast :: RecordPat , token : SyntaxToken , edition : Edition , display_target : DisplayTarget ,) -> Option < SignatureHelp > { signature_help_for_record_ (sema , record . record_pat_field_list () ? . syntax () . children_with_tokens () , & record . path () ? , record . record_pat_field_list () ? . fields () . filter_map (| field | sema . resolve_record_pat_field (& field)) , token , edition , display_target ,) }
};
}
