// Generated macro for macro_10270 (macro)
macro_rules! Depcrate_typesmacro_10270 {
() => {
// Module: crate::types
// Provides: {"macro_10270"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Rc<T>` and `Arc<T>` when `T` is a mutable buffer type such as `String` or `Vec`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Expressions such as `Rc<String>` usually have no advantage over `Rc<str>`, since"] # [doc = " it is larger and involves an extra level of indirection, and doesn't implement `Borrow<str>`."] # [doc = ""] # [doc = " While mutating a buffer type would still be possible with `Rc::get_mut()`, it only"] # [doc = " works if there are no additional references yet, which usually defeats the purpose of"] # [doc = " enclosing it in a shared ownership type. Instead, additionally wrapping the inner"] # [doc = " type with an interior mutable container (such as `RefCell` or `Mutex`) would normally"] # [doc = " be used."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This pattern can be desirable to avoid the overhead of a `RefCell` or `Mutex` for"] # [doc = " cases where mutation only happens before there are any additional references."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " # use std::rc::Rc;"] # [doc = " fn foo(interned: Rc<String>) { ... }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn foo(interned: Rc<str>) { ... }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub RC_BUFFER , restriction , "shared ownership of a buffer type" }
};
}
