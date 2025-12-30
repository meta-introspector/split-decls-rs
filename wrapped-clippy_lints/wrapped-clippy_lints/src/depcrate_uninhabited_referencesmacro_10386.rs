// Generated macro for macro_10386 (macro)
macro_rules! Depcrate_uninhabited_referencesmacro_10386 {
() => {
// Module: crate::uninhabited_references
// Provides: {"macro_10386"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It detects references to uninhabited types, such as `!` and"] # [doc = " warns when those are either dereferenced or returned from a function."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Dereferencing a reference to an uninhabited type would create"] # [doc = " an instance of such a type, which cannot exist. This constitutes"] # [doc = " undefined behaviour. Such a reference could have been created"] # [doc = " by `unsafe` code."] # [doc = ""] # [doc = " ### Example"] # [doc = " The following function can return a reference to an uninhabited type"] # [doc = " (`Infallible`) because it uses `unsafe` code to create it. However,"] # [doc = " the user of such a function could dereference the return value and"] # [doc = " trigger an undefined behavior from safe code."] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn create_ref() -> &'static std::convert::Infallible {"] # [doc = "     unsafe { std::mem::transmute(&()) }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub UNINHABITED_REFERENCES , nursery , "reference to uninhabited type" }
};
}
