// Generated macro for format_doc_comments (function)
macro_rules! Depcrate_jsformat_doc_comments {
() => {
// Module: crate::js
// Provides: {"format_doc_comments"}
// Dependencies: {}
fn format_doc_comments (comments : & str , js_doc_comments : Option < String >) -> String { let body : String = comments . lines () . fold (String :: new () , | mut output , c | { output . push_str (" *") ; if ! c . is_empty () && ! c . starts_with (' ') { output . push (' ') ; } output . push_str (c) ; output . push ('\n') ; output }) ; let doc = if let Some (docs) = js_doc_comments { docs . lines () . fold (String :: new () , | mut output : String , l | { let _ = writeln ! (output , " * {l}") ; output }) } else { String :: new () } ; if body . is_empty () && doc . is_empty () { String :: new () } else { format ! ("/**\n{body}{doc} */\n") } }
};
}
