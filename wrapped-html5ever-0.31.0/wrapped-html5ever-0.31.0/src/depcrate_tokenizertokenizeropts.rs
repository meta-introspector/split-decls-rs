// Generated macro for TokenizerOpts (struct)
macro_rules! Depcrate_tokenizerTokenizerOpts {
() => {
// Module: crate::tokenizer
// Provides: {"TokenizerOpts"}
// Dependencies: {}
# [doc = " Tokenizer options, with an impl for `Default`."] # [derive (Clone)] pub struct TokenizerOpts { # [doc = " Report all parse errors described in the spec, at some"] # [doc = " performance penalty?  Default: false"] pub exact_errors : bool , # [doc = " Discard a `U+FEFF BYTE ORDER MARK` if we see one at the beginning"] # [doc = " of the stream?  Default: true"] pub discard_bom : bool , # [doc = " Keep a record of how long we spent in each state?  Printed"] # [doc = " when `end()` is called.  Default: false"] pub profile : bool , # [doc = " Initial state override.  Only the test runner should use"] # [doc = " a non-`None` value!"] pub initial_state : Option < states :: State > , # [doc = " Last start tag.  Only the test runner should use a"] # [doc = " non-`None` value!"] # [doc = ""] # [doc = " FIXME: Can't use Tendril because we want TokenizerOpts"] # [doc = " to be Send."] pub last_start_tag_name : Option < String > , }
};
}
