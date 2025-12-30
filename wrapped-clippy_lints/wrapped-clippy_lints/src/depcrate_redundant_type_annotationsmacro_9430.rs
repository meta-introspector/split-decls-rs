// Generated macro for macro_9430 (macro)
macro_rules! Depcrate_redundant_type_annotationsmacro_9430 {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"macro_9430"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns about needless / redundant type annotations."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Code without type annotations is shorter and in most cases"] # [doc = " more idiomatic and easier to modify."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " This lint doesn't support:"] # [doc = ""] # [doc = " - Generics"] # [doc = " - Refs returned from anything else than a `MethodCall`"] # [doc = " - Complex types (tuples, arrays, etc...)"] # [doc = " - `Path` to anything else than a primitive type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let foo: String = String::new();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let foo = String::new();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub REDUNDANT_TYPE_ANNOTATIONS , restriction , "warns about needless / redundant type annotations." }
};
}
