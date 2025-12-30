// Generated macro for ErrorContext (trait)
macro_rules! Depcrate_errorErrorContext {
() => {
// Module: crate::error
// Provides: {"ErrorContext"}
// Dependencies: {}
# [doc = " A trait for contextualizing error values."] # [doc = ""] # [doc = " This makes it easy to contextualize either `Error` or `Result<T, Error>`."] # [doc = " Specifically, in the latter case, it absolves one of the need to call"] # [doc = " `map_err` everywhere one wants to add context to an error."] # [doc = ""] # [doc = " This trick was borrowed from `anyhow`."] pub (crate) trait ErrorContext { # [doc = " Contextualize the given consequent error with this (`self`) error as"] # [doc = " the cause."] # [doc = ""] # [doc = " This is equivalent to saying that \"consequent is caused by self.\""] # [doc = ""] # [doc = " Note that if an `Error` is given for `kind`, then this panics if it has"] # [doc = " a cause. (Because the cause would otherwise be dropped. An error causal"] # [doc = " chain is just a linked list, not a tree.)"] fn context (self , consequent : impl IntoError) -> Self ; # [doc = " Like `context`, but hides error construction within a closure."] # [doc = ""] # [doc = " This is useful if the creation of the consequent error is not otherwise"] # [doc = " guarded and when error construction is potentially \"costly\" (i.e., it"] # [doc = " allocates). The closure avoids paying the cost of contextual error"] # [doc = " creation in the happy path."] # [doc = ""] # [doc = " Usually this only makes sense to use on a `Result<T, Error>`, otherwise"] # [doc = " the closure is just executed immediately anyway."] fn with_context < E : IntoError > (self , consequent : impl FnOnce () -> E ,) -> Self ; }
};
}
