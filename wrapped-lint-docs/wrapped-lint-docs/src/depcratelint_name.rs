// Generated macro for lint_name (function)
macro_rules! Depcratelint_name {
() => {
// Module: crate
// Provides: {"lint_name"}
// Dependencies: {}
# [doc = " Extracts the lint name (removing the visibility modifier, and checking validity)."] fn lint_name (line : & str) -> Result < String , & 'static str > { match line . trim () . split (' ') . next_back () { Some (name) => { if ! name . ends_with (',') { return Err ("lint name should end with comma") ; } let name = & name [.. name . len () - 1] ; if ! name . chars () . all (| ch | ch . is_uppercase () || ch . is_ascii_digit () || ch == '_') || name . is_empty () { return Err ("lint name did not have expected format") ; } Ok (name . to_lowercase () . to_string ()) } None => Err ("could not find lint name") , } }
};
}
