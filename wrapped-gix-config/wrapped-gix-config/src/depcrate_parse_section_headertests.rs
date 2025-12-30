// Generated macro for tests (module)
macro_rules! Depcrate_parse_section_headertests {
() => {
// Module: crate::parse::section::header
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn empty_header_names_are_legal () { assert ! (Header :: new ("" , None) . is_ok () , "yes, git allows this, so do we") ; } # [test] fn empty_header_sub_names_are_legal () { assert ! (Header :: new ("remote" , Some (Cow :: Borrowed ("" . into ()))) . is_ok () , "yes, git allows this, so do we") ; } }
};
}
