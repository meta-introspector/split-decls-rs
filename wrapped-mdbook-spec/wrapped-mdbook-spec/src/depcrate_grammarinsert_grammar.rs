// Generated macro for insert_grammar (function)
macro_rules! Depcrate_grammarinsert_grammar {
() => {
// Module: crate::grammar
// Provides: {"insert_grammar"}
// Dependencies: {}
# [doc = " Replaces the text grammar in the given chapter with the rendered version."] pub fn insert_grammar (grammar : & Grammar , chapter : & Chapter , diag : & mut Diagnostics) -> String { let link_map = make_relative_link_map (grammar , chapter) ; let mut content = GRAMMAR_RE . replace_all (& chapter . content , | cap : & Captures < '_ > | { let names : Vec < _ > = NAMES_RE . captures_iter (& cap [2]) . map (| cap | cap . get (1) . unwrap () . as_str ()) . collect () ; let for_lexer = & cap [1] == "lexer" ; render_names (grammar , & names , & link_map , for_lexer , chapter , diag) }) . to_string () ; let is_summary = is_summary (chapter) ; for (name , path) in & link_map { let id = render_markdown :: markdown_id (name , is_summary) ; if is_summary { writeln ! (content , "[{name}]: #{id}") . unwrap () ; } else { writeln ! (content , "[{name}]: {path}#{id}\n\
                 [grammar-{name}]: {path}#{id}") . unwrap () ; } } content }
};
}
