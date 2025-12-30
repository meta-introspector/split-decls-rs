// Generated macro for Spans (struct)
macro_rules! Depcrate_errorSpans {
() => {
// Module: crate::error
// Provides: {"Spans"}
// Dependencies: {}
# [doc = " This type represents an arbitrary number of error spans in a way that makes"] # [doc = " it convenient to notate the regex pattern. (\"Notate\" means \"point out"] # [doc = " exactly where the error occurred in the regex pattern.\")"] # [doc = ""] # [doc = " Technically, we can only ever have two spans given our current error"] # [doc = " structure. However, after toiling with a specific algorithm for handling"] # [doc = " two spans, it became obvious that an algorithm to handle an arbitrary"] # [doc = " number of spans was actually much simpler."] struct Spans < 'p > { # [doc = " The original regex pattern string."] pattern : & 'p str , # [doc = " The total width that should be used for line numbers. The width is"] # [doc = " used for left padding the line numbers for alignment."] # [doc = ""] # [doc = " A value of `0` means line numbers should not be displayed. That is,"] # [doc = " the pattern is itself only one line."] line_number_width : usize , # [doc = " All error spans that occur on a single line. This sequence always has"] # [doc = " length equivalent to the number of lines in `pattern`, where the index"] # [doc = " of the sequence represents a line number, starting at `0`. The spans"] # [doc = " in each line are sorted in ascending order."] by_line : Vec < Vec < ast :: Span > > , # [doc = " All error spans that occur over one or more lines. That is, the start"] # [doc = " and end position of the span have different line numbers. The spans are"] # [doc = " sorted in ascending order."] multi_line : Vec < ast :: Span > , }
};
}
