// Generated macro for Formatter (struct)
macro_rules! Depcrate_errorFormatter {
() => {
// Module: crate::error
// Provides: {"Formatter"}
// Dependencies: {}
# [doc = " A helper type for formatting nice error messages."] # [doc = ""] # [doc = " This type is responsible for reporting regex parse errors in a nice human"] # [doc = " readable format. Most of its complexity is from interspersing notational"] # [doc = " markers pointing out the position where an error occurred."] # [derive (Debug)] pub struct Formatter < 'e , E > { # [doc = " The original regex pattern in which the error occurred."] pattern : & 'e str , # [doc = " The error kind. It must impl fmt::Display."] err : & 'e E , # [doc = " The primary span of the error."] span : & 'e ast :: Span , # [doc = " An auxiliary and optional span, in case the error needs to point to"] # [doc = " two locations (e.g., when reporting a duplicate capture group name)."] aux_span : Option < & 'e ast :: Span > , }
};
}
