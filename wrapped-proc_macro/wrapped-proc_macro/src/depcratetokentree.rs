// Generated macro for TokenTree (enum)
macro_rules! DepcrateTokenTree {
() => {
// Module: crate
// Provides: {"TokenTree"}
// Dependencies: {}
# [doc = " A single token or a delimited sequence of token trees (e.g., `[1, (), ..]`)."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] # [derive (Clone)] pub enum TokenTree { # [doc = " A token stream surrounded by bracket delimiters."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Group (# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Group) , # [doc = " An identifier."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Ident (# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Ident) , # [doc = " A single punctuation character (`+`, `,`, `$`, etc.)."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Punct (# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Punct) , # [doc = " A literal character (`'a'`), string (`\"hello\"`), number (`2.3`), etc."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Literal (# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] Literal) , }
};
}
