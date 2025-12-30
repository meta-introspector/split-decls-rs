// Generated macro for macro_8794 (macro)
macro_rules! Depcrate_ptrmacro_8794 {
() => {
// Module: crate::ptr
// Provides: {"macro_8794"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint checks for function arguments of type `&String`, `&Vec`,"] # [doc = " `&PathBuf`, and `Cow<_>`. It will also suggest you replace `.clone()` calls"] # [doc = " with the appropriate `.to_owned()`/`to_string()` calls."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Requiring the argument to be of the specific type"] # [doc = " makes the function less useful for no benefit; slices in the form of `&[T]`"] # [doc = " or `&str` usually suffice and can be obtained from other types, too."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " There may be `fn(&Vec)`-typed references pointing to your function."] # [doc = " If you have them, you will get a compiler error after applying this lint's"] # [doc = " suggestions. You then have the choice to undo your changes or change the"] # [doc = " type of the reference."] # [doc = ""] # [doc = " Note that if the function is part of your public interface, there may be"] # [doc = " other crates referencing it, of which you may not be aware. Carefully"] # [doc = " deprecate the function before applying the lint suggestions in this case."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " fn foo(&Vec<u32>) { .. }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " fn foo(&[u32]) { .. }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PTR_ARG , style , "fn arguments of the type `&Vec<...>` or `&String`, suggesting to use `&[...]` or `&str` instead, respectively" }
};
}
