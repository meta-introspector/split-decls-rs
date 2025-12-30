// Generated macro for macro_1444 (macro)
macro_rules! Depcrate_dereferencemacro_1444 {
() => {
// Module: crate::dereference
// Provides: {"macro_1444"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for explicit `deref()` or `deref_mut()` method calls."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Dereferencing by `&*x` or `&mut *x` is clearer and more concise,"] # [doc = " when not part of a method chain."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::ops::Deref;"] # [doc = " let a: &mut String = &mut String::from(\"foo\");"] # [doc = " let b: &str = a.deref();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a: &mut String = &mut String::from(\"foo\");"] # [doc = " let b = &*a;"] # [doc = " ```"] # [doc = ""] # [doc = " This lint excludes all of:"] # [doc = " ```rust,ignore"] # [doc = " let _ = d.unwrap().deref();"] # [doc = " let _ = Foo::deref(&foo);"] # [doc = " let _ = <Foo as Deref>::deref(&foo);"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub EXPLICIT_DEREF_METHODS , pedantic , "Explicit use of deref or deref_mut method while not in a method chain." }
};
}
