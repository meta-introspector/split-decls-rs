// Generated macro for write_padded_newline (function)
macro_rules! Depcrate_text_modificationswrite_padded_newline {
() => {
// Module: crate::text_modifications
// Provides: {"write_padded_newline"}
// Dependencies: {}
# [doc = " Write a newline followed by the current [`State::padding`]"] # [doc = " text that indents the current nested content."] # [doc = ""] # [doc = " [`write_padded_newline()`] takes care of writing both a newline character,"] # [doc = " and the appropriate padding characters, abstracting over the need to"] # [doc = " carefully pair those actions."] # [doc = ""] # [doc = " # Purpose"] # [doc = ""] # [doc = " Consider a scenario where we're trying to write out the following Markdown"] # [doc = " (space indents visualized as '·'):"] # [doc = ""] # [doc = " ```markdown"] # [doc = " >·A block quote with an embedded list:"] # [doc = " >·"] # [doc = " >·* This is a list item that itself contains"] # [doc = " >···multiple lines and paragraphs of content."] # [doc = " >···"] # [doc = " >···Second paragraph."] # [doc = " ```"] # [doc = ""] # [doc = " Each line of output within the block quote needs to include the text `\">·\"`"] # [doc = " at the beginning of the line. Additionally, within the list, each line"] # [doc = " _also_ needs to start with `\"··\"` spaces so that the content of the"] # [doc = " list item is indented."] # [doc = ""] # [doc = " Concretely, a call to [`write_padded_newline()`] after the first line in the"] # [doc = " paragraph of the list item would write `\"\\n>···\"`."] pub (crate) fn write_padded_newline (formatter : & mut impl fmt :: Write , state : & State < '_ >) -> Result < () , fmt :: Error > { formatter . write_char ('\n') ? ; padding (formatter , & state . padding) ? ; Ok (()) }
};
}
