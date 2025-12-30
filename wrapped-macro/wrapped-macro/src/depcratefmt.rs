// Generated macro for fmt (function)
macro_rules! Depcratefmt {
() => {
// Module: crate
// Provides: {"fmt"}
// Dependencies: {}
# [doc = " Format a valid Rust string"] fn fmt (input : & str) -> Result < String > { let syntax_tree = syn :: parse_file (& input) ? ; Ok (prettyplease :: unparse (& syntax_tree)) }
};
}
