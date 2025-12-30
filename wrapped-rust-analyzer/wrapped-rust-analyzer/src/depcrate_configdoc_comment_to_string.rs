// Generated macro for doc_comment_to_string (function)
macro_rules! Depcrate_configdoc_comment_to_string {
() => {
// Module: crate::config
// Provides: {"doc_comment_to_string"}
// Dependencies: {}
fn doc_comment_to_string (doc : & [& str]) -> String { doc . iter () . map (| it | it . strip_prefix (' ') . unwrap_or (it)) . fold (String :: new () , | mut acc , it | format_to_acc ! (acc , "{it}\n")) }
};
}
