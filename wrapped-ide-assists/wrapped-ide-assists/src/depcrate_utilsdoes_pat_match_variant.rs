// Generated macro for does_pat_match_variant (function)
macro_rules! Depcrate_utilsdoes_pat_match_variant {
() => {
// Module: crate::utils
// Provides: {"does_pat_match_variant"}
// Dependencies: {}
pub (crate) fn does_pat_match_variant (pat : & ast :: Pat , var : & ast :: Pat) -> bool { let first_node_text = | pat : & ast :: Pat | pat . syntax () . first_child () . map (| node | node . text ()) ; let pat_head = match pat { ast :: Pat :: IdentPat (bind_pat) => match bind_pat . pat () { Some (p) => first_node_text (& p) , None => return pat . syntax () . text () == var . syntax () . text () , } , pat => first_node_text (pat) , } ; let var_head = first_node_text (var) ; pat_head == var_head }
};
}
