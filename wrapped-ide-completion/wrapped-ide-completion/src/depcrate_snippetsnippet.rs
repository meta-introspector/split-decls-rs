// Generated macro for Snippet (struct)
macro_rules! Depcrate_snippetSnippet {
() => {
// Module: crate::snippet
// Provides: {"Snippet"}
// Dependencies: {}
# [doc = " A user supplied snippet."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Snippet { pub postfix_triggers : Box < [Box < str >] > , pub prefix_triggers : Box < [Box < str >] > , pub scope : SnippetScope , pub description : Option < Box < str > > , snippet : String , requires : Box < [ModPath] > , }
};
}
