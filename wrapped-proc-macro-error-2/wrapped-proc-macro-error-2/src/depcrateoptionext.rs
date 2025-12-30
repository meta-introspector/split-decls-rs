// Generated macro for OptionExt (trait)
macro_rules! DepcrateOptionExt {
() => {
// Module: crate
// Provides: {"OptionExt"}
// Dependencies: {}
# [doc = " This traits expands `Option` with some handy shortcuts."] pub trait OptionExt { type Some ; # [doc = " Behaves like `Option::expect`: if self is `Some` yield the contained value,"] # [doc = " otherwise abort macro execution via `abort_call_site!`."] # [doc = " If it aborts the `message` will be used for [`compile_error!`][compl_err] invocation."] # [doc = ""] # [doc = " [compl_err]: https://doc.rust-lang.org/std/macro.compile_error.html"] fn expect_or_abort (self , msg : & str) -> Self :: Some ; }
};
}
