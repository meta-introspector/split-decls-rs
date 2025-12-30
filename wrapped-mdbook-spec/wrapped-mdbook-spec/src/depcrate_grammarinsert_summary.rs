// Generated macro for insert_summary (function)
macro_rules! Depcrate_grammarinsert_summary {
() => {
// Module: crate::grammar
// Provides: {"insert_summary"}
// Dependencies: {}
# [doc = " Inserts the summary of all grammar rules into the grammar summary chapter."] pub fn insert_summary (grammar : & Grammar , chapter : & Chapter , diag : & mut Diagnostics) -> String { let link_map = make_relative_link_map (grammar , chapter) ; let mut seen = HashSet :: new () ; let categories : Vec < _ > = grammar . name_order . iter () . map (| name | & grammar . productions [name] . category) . filter (| cat | seen . insert (* cat)) . collect () ; let mut grammar_summary = String :: new () ; for category in categories { let mut chars = category . chars () ; let cap = chars . next () . unwrap () . to_uppercase () . collect :: < String > () + chars . as_str () ; write ! (grammar_summary , "\n## {cap} summary\n\n") . unwrap () ; let names : Vec < _ > = grammar . name_order . iter () . filter (| name | grammar . productions [* name] . category == * category) . map (| s | s . as_str ()) . collect () ; let for_lexer = category == "lexer" ; let s = render_names (grammar , & names , & link_map , for_lexer , chapter , diag) ; grammar_summary . push_str (& s) ; } chapter . content . replace ("{{ grammar-summary }}" , & grammar_summary) }
};
}
