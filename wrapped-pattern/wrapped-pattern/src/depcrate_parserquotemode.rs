// Generated macro for QuoteMode (enum)
macro_rules! Depcrate_parserQuoteMode {
() => {
// Module: crate::parser
// Provides: {"QuoteMode"}
// Dependencies: {}
# [doc = " Controls how quotes (`'`) are interpreted."] # [derive (Debug , Default , PartialEq)] # [non_exhaustive] pub enum QuoteMode { # [doc = " Quotes are interpreted as literals, i.e. `{0} o'clock` will interpolate to `5 o'clock`."] # [default] QuotesAreLiterals , # [doc = " Quotes can be used to quote ASCII characters, i.e. both `{0} World` and `{0} 'World'` will interpolate to `Hello World`."] # [doc = ""] # [doc = " A double quote can be used to create a quote literal, i.e. `{0} o''clock`."] QuotingSupported , # [doc = " Quotes are required to quote ASCII characters, i.e. `{0} 'World'` will interpolate to `Hello World`, while `{0} World` is an error."] # [doc = ""] # [doc = " A double quote can be used to create a quote literal, i.e. `{0} 'o''clock'`."] QuotingRequired , }
};
}
