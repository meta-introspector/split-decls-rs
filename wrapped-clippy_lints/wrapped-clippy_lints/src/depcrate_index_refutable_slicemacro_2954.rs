// Generated macro for macro_2954 (macro)
macro_rules! Depcrate_index_refutable_slicemacro_2954 {
() => {
// Module: crate::index_refutable_slice
// Provides: {"macro_2954"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " The lint checks for slice bindings in patterns that are only used to"] # [doc = " access individual slice values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Accessing slice values using indices can lead to panics. Using refutable"] # [doc = " patterns can avoid these. Binding to individual values also improves the"] # [doc = " readability as they can be named."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " This lint currently only checks for immutable access inside `if let`"] # [doc = " patterns."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let slice: Option<&[u32]> = Some(&[1, 2, 3]);"] # [doc = ""] # [doc = " if let Some(slice) = slice {"] # [doc = "     println!(\"{}\", slice[0]);"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let slice: Option<&[u32]> = Some(&[1, 2, 3]);"] # [doc = ""] # [doc = " if let Some(&[first, ..]) = slice {"] # [doc = "     println!(\"{}\", first);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.59.0"] pub INDEX_REFUTABLE_SLICE , pedantic , "avoid indexing on slices which could be destructed" }
};
}
