// Generated macro for set_suggestion (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineeset_suggestion {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"set_suggestion"}
// Dependencies: {}
fn set_suggestion < 'tcx > (diag : & mut Diag < '_ , () > , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , found : FoundSigDrop) { let original = snippet (cx , found . found_span , "..") ; let trailing_indent = " " . repeat (indent_of (cx , found . found_span) . unwrap_or (0)) ; let replacement = { let (def_part , deref_part) = if found . is_unit_return_val { ("" , String :: new ()) } else { ("let value = " , "*" . repeat (found . peel_ref_times)) } ; format ! ("{def_part}{deref_part}{original};\n{trailing_indent}") } ; let suggestion_message = if found . peel_ref_times == 0 { "try moving the temporary above the match" } else { "try moving the temporary above the match and create a copy" } ; let scrutinee_replacement = if found . is_unit_return_val { "()" . to_owned () } else if found . peel_ref_times == 0 { "value" . to_owned () } else { let ref_part = "&" . repeat (found . peel_ref_times) ; format ! ("({ref_part}value)") } ; diag . multipart_suggestion (suggestion_message , vec ! [(first_line_of_span (cx , expr . span) . shrink_to_lo () , replacement) , (found . found_span , scrutinee_replacement) ,] , Applicability :: MaybeIncorrect ,) ; }
};
}
