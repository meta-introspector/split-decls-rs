// Generated macro for Capture (struct)
macro_rules! Depcrate_hirCapture {
() => {
// Module: crate::hir
// Provides: {"Capture"}
// Dependencies: {}
# [doc = " The high-level intermediate representation for a capturing group."] # [doc = ""] # [doc = " A capturing group always has an index and a child expression. It may"] # [doc = " also have a name associated with it (e.g., `(?P<foo>\\w)`), but it's not"] # [doc = " necessary."] # [doc = ""] # [doc = " Note that there is no explicit representation of a non-capturing group"] # [doc = " in a `Hir`. Instead, non-capturing grouping is handled automatically by"] # [doc = " the recursive structure of the `Hir` itself."] # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct Capture { # [doc = " The capture index of the capture."] pub (crate) index : u32 , # [doc = " The name of the capture, if it exists."] pub (crate) name : Option < Box < str > > , # [doc = " The expression inside the capturing group, which may be empty."] pub (crate) sub : Box < Hir > , }
};
}
