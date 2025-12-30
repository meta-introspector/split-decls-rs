// Generated macro for TokenCursor (struct)
macro_rules! Depcrate_tokenstreamTokenCursor {
() => {
// Module: crate::tokenstream
// Provides: {"TokenCursor"}
// Dependencies: {}
# [doc = " A `TokenStream` cursor that produces `Token`s. It's a bit odd that"] # [doc = " we (a) lex tokens into a nice tree structure (`TokenStream`), and then (b)"] # [doc = " use this type to emit them as a linear sequence. But a linear sequence is"] # [doc = " what the parser expects, for the most part."] # [derive (Clone , Debug)] pub struct TokenCursor { pub curr : TokenTreeCursor , pub stack : Vec < TokenTreeCursor > , }
};
}
