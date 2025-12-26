use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The parser structure for interpreting the input format string. This is
/// modeled as an iterator over `Piece` structures to form a stream of tokens
/// being output.
///
/// This is a recursive-descent parser for the sake of simplicity, and if
/// necessary there's probably lots of room for improvement performance-wise.
pub struct Parser<'input> {
    mode: ParseMode,
    /// Input to be parsed
    input: &'input str,
    /// Tuples of the span in the code snippet (input as written before being unescaped), the pos in input, and the char in input
    input_vec: Vec<(Range<usize>, usize, char)>,
    /// Index into input_vec
    input_vec_index: usize,
    /// Error messages accumulated during parsing
    pub errors: Vec<ParseError>,
    /// Current position of implicit positional argument pointer
    pub curarg: usize,
    /// Start and end byte offset of every successfully parsed argument
    pub arg_places: Vec<Range<usize>>,
    /// Span of the last opening brace seen, used for error reporting
    last_open_brace: Option<Range<usize>>,
    /// Whether this formatting string was written directly in the source. This controls whether we
    /// can use spans to refer into it and give better error messages.
    /// N.B: This does _not_ control whether implicit argument captures can be used.
    pub is_source_literal: bool,
    /// Index to the end of the literal snippet
    end_of_snippet: usize,
    /// Start position of the current line.
    cur_line_start: usize,
    /// Start and end byte offset of every line of the format string. Excludes
    /// newline characters and leading whitespace.
    pub line_spans: Vec<Range<usize>>,
}
