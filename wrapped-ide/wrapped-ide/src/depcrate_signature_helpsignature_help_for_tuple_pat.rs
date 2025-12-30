// Generated macro for signature_help_for_tuple_pat (function)
macro_rules! Depcrate_signature_helpsignature_help_for_tuple_pat {
() => {
// Module: crate::signature_help
// Provides: {"signature_help_for_tuple_pat"}
// Dependencies: {}
fn signature_help_for_tuple_pat (sema : & Semantics < '_ , RootDatabase > , pat : ast :: TuplePat , token : SyntaxToken , display_target : DisplayTarget ,) -> Option < SignatureHelp > { let db = sema . db ; let field_pats = pat . fields () ; let pat = pat . into () ; let ty = sema . type_of_pat (& pat) ? ; let fields = ty . original . tuple_fields (db) ; Some (signature_help_for_tuple_pat_ish (db , SignatureHelp { doc : None , signature : String :: from ('(') , parameters : vec ! [] , active_parameter : None , } , pat . syntax () , token , field_pats , fields . into_iter () , display_target ,)) }
};
}
