// Generated macro for Fuse (struct)
macro_rules! Depcrate_fuseFuse {
() => {
// Module: crate::fuse
// Provides: {"Fuse"}
// Dependencies: {}
# [doc = " A future which \"fuse\"s a future once it's been resolved."] # [doc = ""] # [doc = " Normally futures can behave unpredictable once they're used after a future"] # [doc = " has been resolved, but `Fuse` is always defined to return `None` from `poll`"] # [doc = " after it has succeeded, and after it has succeeded all future calls to"] # [doc = " `schedule` will be ignored."] pub struct Fuse < A > { future : Option < A > , }
};
}
