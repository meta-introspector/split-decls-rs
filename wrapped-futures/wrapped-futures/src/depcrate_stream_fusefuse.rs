// Generated macro for Fuse (struct)
macro_rules! Depcrate_stream_fuseFuse {
() => {
// Module: crate::stream::fuse
// Provides: {"Fuse"}
// Dependencies: {}
# [doc = " A stream which \"fuse\"s a stream once it's terminated."] # [doc = ""] # [doc = " Normally streams can behave unpredictably after they've terminated or"] # [doc = " returned an error, but `Fuse` is always defined to return `None` from `poll`"] # [doc = " after terination/errors, and afterwards all calls to `schedule` will be"] # [doc = " ignored."] pub struct Fuse < S > { stream : Option < S > , }
};
}
