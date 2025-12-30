// Generated macro for stream (macro)
macro_rules! Depcratestream {
() => {
// Module: crate
// Provides: {"stream"}
// Dependencies: {}
# [doc = " Asynchronous stream"] # [doc = ""] # [doc = " See [crate](index.html) documentation for more details."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_stream::stream;"] # [doc = ""] # [doc = " use futures_util::pin_mut;"] # [doc = " use futures_util::stream::StreamExt;"] # [doc = ""] # [doc = " #[tokio::main]"] # [doc = " async fn main() {"] # [doc = "     let s = stream! {"] # [doc = "         for i in 0..3 {"] # [doc = "             yield i;"] # [doc = "         }"] # [doc = "     };"] # [doc = ""] # [doc = "     pin_mut!(s); // needed for iteration"] # [doc = ""] # [doc = "     while let Some(value) = s.next().await {"] # [doc = "         println!(\"got {}\", value);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! stream { ($ ($ tt : tt) *) => { $ crate :: __private :: stream_inner ! (($ crate) $ ($ tt) *) } }
};
}
