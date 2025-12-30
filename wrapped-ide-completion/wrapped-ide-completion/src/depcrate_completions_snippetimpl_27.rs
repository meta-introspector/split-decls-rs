// Generated macro for impl_27 (impl)
macro_rules! Depcrate_completions_snippetimpl_27 {
() => {
// Module: crate::completions::snippet
// Provides: {"impl_27"}
// Dependencies: {}
impl Snippet { pub fn new (prefix_triggers : & [String] , postfix_triggers : & [String] , snippet : & [String] , description : & str , requires : & [String] , scope : SnippetScope ,) -> Option < Self > { if prefix_triggers . is_empty () && postfix_triggers . is_empty () { return None ; } let (requires , snippet , description) = validate_snippet (snippet , description , requires) ? ; Some (Snippet { postfix_triggers : postfix_triggers . iter () . map (String :: as_str) . map (Into :: into) . collect () , prefix_triggers : prefix_triggers . iter () . map (String :: as_str) . map (Into :: into) . collect () , scope , snippet , description , requires , }) } # [doc = " Returns [`None`] if the required items do not resolve."] pub (crate) fn imports (& self , ctx : & CompletionContext < '_ >) -> Option < Vec < LocatedImport > > { import_edits (ctx , & self . requires) } pub fn snippet (& self) -> String { self . snippet . replace ("${receiver}" , "$0") } pub fn postfix_snippet (& self , receiver : & str) -> String { self . snippet . replace ("${receiver}" , receiver) } }
};
}
