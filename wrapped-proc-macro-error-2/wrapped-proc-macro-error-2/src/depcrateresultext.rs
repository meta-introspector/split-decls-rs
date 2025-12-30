// Generated macro for ResultExt (trait)
macro_rules! DepcrateResultExt {
() => {
// Module: crate
// Provides: {"ResultExt"}
// Dependencies: {}
# [doc = " This traits expands `Result<T, Into<Diagnostic>>` with some handy shortcuts."] pub trait ResultExt { type Ok ; # [doc = " Behaves like `Result::unwrap`: if self is `Ok` yield the contained value,"] # [doc = " otherwise abort macro execution via `abort!`."] fn unwrap_or_abort (self) -> Self :: Ok ; # [doc = " Behaves like `Result::expect`: if self is `Ok` yield the contained value,"] # [doc = " otherwise abort macro execution via `abort!`."] # [doc = " If it aborts then resulting error message will be preceded with `message`."] fn expect_or_abort (self , msg : & str) -> Self :: Ok ; }
};
}
