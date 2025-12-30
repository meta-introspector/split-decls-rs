// Generated macro for Fragment (enum)
macro_rules! Depcrate_expanderFragment {
() => {
// Module: crate::expander
// Provides: {"Fragment"}
// Dependencies: {}
# [derive (Debug , Default , Clone)] enum Fragment < 'a > { # [default] Empty , # [doc = " token fragments are just copy-pasted into the output"] Tokens (tt :: TokenTreesView < 'a , Span >) , # [doc = " Expr ast fragments are surrounded with `()` on transcription to preserve precedence."] # [doc = " Note that this impl is different from the one currently in `rustc` --"] # [doc = " `rustc` doesn't translate fragments into token trees at all."] # [doc = ""] # [doc = " At one point in time, we tried to use \"fake\" delimiters here à la"] # [doc = " proc-macro delimiter=none. As we later discovered, \"none\" delimiters are"] # [doc = " tricky to handle in the parser, and rustc doesn't handle those either."] # [doc = ""] # [doc = " The span of the outer delimiters is marked on transcription."] Expr (tt :: TokenTreesView < 'a , Span >) , # [doc = " There are roughly two types of paths: paths in expression context, where a"] # [doc = " separator `::` between an identifier and its following generic argument list"] # [doc = " is mandatory, and paths in type context, where `::` can be omitted."] # [doc = ""] # [doc = " Unlike rustc, we need to transform the parsed fragments back into tokens"] # [doc = " during transcription. When the matched path fragment is a type-context path"] # [doc = " and is trasncribed as an expression-context path, verbatim transcription"] # [doc = " would cause a syntax error. We need to fix it up just before transcribing;"] # [doc = " see `transcriber::fix_up_and_push_path_tt()`."] Path (tt :: TokenTreesView < 'a , Span >) , TokensOwned (tt :: TopSubtree < Span >) , }
};
}
