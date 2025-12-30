// Generated macro for stream_select (macro)
macro_rules! Depcrate_async_await_stream_select_modstream_select {
() => {
// Module: crate::async_await::stream_select_mod
// Provides: {"stream_select"}
// Dependencies: {}
# [allow (clippy :: too_long_first_doc_paragraph)] # [doc = " Combines several streams, all producing the same `Item` type, into one stream."] # [doc = " This is similar to `select_all` but does not require the streams to all be the same type."] # [doc = " It also keeps the streams inline, and does not require `Box<dyn Stream>`s to be allocated."] # [doc = " Streams passed to this macro must be `Unpin`."] # [doc = ""] # [doc = " If multiple streams are ready, one will be pseudo randomly selected at runtime."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::{stream, StreamExt, stream_select};"] # [doc = " let endless_ints = |i| stream::iter(vec![i].into_iter().cycle()).fuse();"] # [doc = ""] # [doc = " let mut endless_numbers = stream_select!(endless_ints(1i32), endless_ints(2), endless_ints(3));"] # [doc = " match endless_numbers.next().await {"] # [doc = "     Some(1) => println!(\"Got a 1\"),"] # [doc = "     Some(2) => println!(\"Got a 2\"),"] # [doc = "     Some(3) => println!(\"Got a 3\"),"] # [doc = "     _ => unreachable!(),"] # [doc = " }"] # [doc = " # });"] # [doc = " ```"] # [macro_export] macro_rules ! stream_select { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: stream_select_internal ! { $ ($ tokens) * } } } }
};
}
