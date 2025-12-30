// Generated macro for extract_tokens_from_content (function)
macro_rules! Depcrateextract_tokens_from_content {
() => {
// Module: crate
// Provides: {"extract_tokens_from_content"}
// Dependencies: {}
fn extract_tokens_from_content (content : & str) -> Result < HashSet < String > , Box < dyn std :: error :: Error > > { if let Ok (ast) = syn :: parse_file (content) { let mut visitor = TokenVisitor { uses : HashSet :: new () } ; visitor . visit_file (& ast) ; Ok (visitor . uses) } else { Ok (HashSet :: new ()) } }
};
}
