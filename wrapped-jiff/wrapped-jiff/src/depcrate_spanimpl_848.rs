// Generated macro for impl_848 (impl)
macro_rules! Depcrate_spanimpl_848 {
() => {
// Module: crate::span
// Provides: {"impl_848"}
// Dependencies: {}
impl < 'a > RelativeSpanKind < 'a > { # [doc = " Create a balanced `RelativeSpan` from this range of time."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This returns an error when the span in this range cannot be"] # [doc = " represented. In general, this only occurs when asking for largest units"] # [doc = " of `Unit::Nanosecond` *and* when the span is too big to fit into a"] # [doc = " 64-bit nanosecond count."] fn into_relative_span (self , largest : Unit ,) -> Result < RelativeSpan < 'a > , Error > { let span = match self { RelativeSpanKind :: Civil { ref start , ref end } => start . datetime . until ((largest , end . datetime)) . with_context (| | { err ! ("failed to get span between {start} and {end} \
                         with largest unit as {unit}" , start = start . datetime , end = end . datetime , unit = largest . plural () ,) }) ? , RelativeSpanKind :: Zoned { ref start , ref end } => start . zoned . until ((largest , & * end . zoned)) . with_context (| | { err ! ("failed to get span between {start} and {end} \
                         with largest unit as {unit}" , start = start . zoned , end = end . zoned , unit = largest . plural () ,) }) ? , } ; Ok (RelativeSpan { span , kind : self }) } }
};
}
