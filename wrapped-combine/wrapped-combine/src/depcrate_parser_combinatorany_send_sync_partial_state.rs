// Generated macro for any_send_sync_partial_state (function)
macro_rules! Depcrate_parser_combinatorany_send_sync_partial_state {
() => {
// Module: crate::parser::combinator
// Provides: {"any_send_sync_partial_state"}
// Dependencies: {}
# [doc = " Returns a parser where `P::PartialState` is boxed. Useful as a way to avoid writing the type"] # [doc = " since it can get very large after combining a few parsers."] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use]"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::combinator::{AnySendSyncPartialState, any_send_sync_partial_state};"] # [doc = " # use combine::parser::char::letter;"] # [doc = " # use combine::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn example<Input>() -> impl Parser<Input, Output = (char, char), PartialState = AnySendSyncPartialState>"] # [doc = " where"] # [doc = "     Input: Stream<Token = char>,"] # [doc = " {"] # [doc = "     any_send_sync_partial_state((letter(), letter()))"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     example().easy_parse(\"ab\"),"] # [doc = "     Ok((('a', 'b'), \"\"))"] # [doc = " );"] # [doc = ""] # [doc = " # }"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn any_send_sync_partial_state < Input , P > (p : P) -> AnySendSyncPartialStateParser < P > where Input : Stream , P : Parser < Input > , P :: PartialState : Send + Sync + 'static , { AnySendSyncPartialStateParser (p) }
};
}
