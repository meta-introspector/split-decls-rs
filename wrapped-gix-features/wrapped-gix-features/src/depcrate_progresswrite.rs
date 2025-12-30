// Generated macro for Write (struct)
macro_rules! Depcrate_progressWrite {
() => {
// Module: crate::progress
// Provides: {"Write"}
// Dependencies: {}
# [doc = " A structure passing every [`write`][std::io::Write::write()] call through to the contained Progress instance using [`inc_by(bytes_written)`](Count::inc_by())."] # [doc = ""] # [doc = " This is particularly useful if the final size of the bytes to write is known or can be estimated precisely enough."] pub struct Write < T , P > { # [doc = " The implementor of [`std::io::Write`] to which progress is added"] pub inner : T , # [doc = " The progress instance receiving progress information on each invocation of `reader`"] pub progress : P , }
};
}
