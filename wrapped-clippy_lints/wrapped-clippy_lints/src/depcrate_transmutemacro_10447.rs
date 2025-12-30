// Generated macro for macro_10447 (macro)
macro_rules! Depcrate_transmutemacro_10447 {
() => {
// Module: crate::transmute
// Provides: {"macro_10447"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes between types which do not have a representation defined relative to"] # [doc = " each other."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The results of such a transmute are not defined."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint has had multiple problems in the past and was moved to `nursery`. See issue"] # [doc = " [#8496](https://github.com/rust-lang/rust-clippy/issues/8496) for more details."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo<T>(u32, T);"] # [doc = " let _ = unsafe { core::mem::transmute::<Foo<u32>, Foo<i32>>(Foo(0u32, 0u32)) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[repr(C)]"] # [doc = " struct Foo<T>(u32, T);"] # [doc = " let _ = unsafe { core::mem::transmute::<Foo<u32>, Foo<i32>>(Foo(0u32, 0u32)) };"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub TRANSMUTE_UNDEFINED_REPR , nursery , "transmute to or from a type with an undefined representation" }
};
}
