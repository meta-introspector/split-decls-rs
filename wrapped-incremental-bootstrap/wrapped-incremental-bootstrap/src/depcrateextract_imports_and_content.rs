// Generated macro for extract_imports_and_content (function)
macro_rules! Depcrateextract_imports_and_content {
() => {
// Module: crate
// Provides: {"extract_imports_and_content"}
// Dependencies: {}
fn extract_imports_and_content (content : & str) -> Result < (String , String) , Box < dyn std :: error :: Error > > { if let Ok (ast) = syn :: parse_file (content) { let mut import_visitor = ImportVisitor { imports : Vec :: new () } ; import_visitor . visit_file (& ast) ; let mut non_use_items = Vec :: new () ; for item in & ast . items { if ! matches ! (item , syn :: Item :: Use (_)) { non_use_items . push (quote :: quote ! (# item) . to_string ()) ; } } let imports_str = import_visitor . imports . join ("\n    ") ; let content_str = non_use_items . join ("\n\n    ") ; Ok ((if imports_str . is_empty () { String :: new () } else { format ! ("    {}" , imports_str) } , if content_str . is_empty () { String :: new () } else { format ! ("    {}" , content_str) })) } else { Ok ((String :: new () , format ! ("    {}" , content))) } }
};
}
