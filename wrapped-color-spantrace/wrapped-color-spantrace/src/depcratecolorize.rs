// Generated macro for colorize (function)
macro_rules! Depcratecolorize {
() => {
// Module: crate
// Provides: {"colorize"}
// Dependencies: {}
# [doc = " Display a [`SpanTrace`] with colors and source"] # [doc = ""] # [doc = " This function returns an `impl Display` type which can be then used in place of the original"] # [doc = " SpanTrace when writing it too the screen or buffer."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tracing_error::SpanTrace;"] # [doc = ""] # [doc = " let span_trace = SpanTrace::capture();"] # [doc = " println!(\"{}\", color_spantrace::colorize(&span_trace));"] # [doc = " ```"] # [doc = ""] # [doc = " **Note:** `colorize` sets the global theme implicitly, if it was not set already. So calling `colorize` and then `set_theme` fails"] # [doc = ""] # [doc = " [`SpanTrace`]: https://docs.rs/tracing-error/*/tracing_error/struct.SpanTrace.html"] pub fn colorize (span_trace : & SpanTrace) -> impl fmt :: Display + '_ { let theme = * THEME . get_or_init (Theme :: dark) ; ColorSpanTrace { span_trace , theme } }
};
}
