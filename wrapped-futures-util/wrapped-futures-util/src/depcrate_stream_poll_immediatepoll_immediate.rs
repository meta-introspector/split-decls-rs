// Generated macro for poll_immediate (function)
macro_rules! Depcrate_stream_poll_immediatepoll_immediate {
() => {
// Module: crate::stream::poll_immediate
// Provides: {"poll_immediate"}
// Dependencies: {}
# [doc = " Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it."] # [doc = ""] # [doc = " This is useful when immediacy is more important than waiting for the next item to be ready."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::stream::{self, StreamExt};"] # [doc = " use futures::task::Poll;"] # [doc = ""] # [doc = " let mut r = stream::poll_immediate(Box::pin(stream::iter(1_u32..3)));"] # [doc = " assert_eq!(r.next().await, Some(Poll::Ready(1)));"] # [doc = " assert_eq!(r.next().await, Some(Poll::Ready(2)));"] # [doc = " assert_eq!(r.next().await, None);"] # [doc = ""] # [doc = " let mut p = stream::poll_immediate(Box::pin(stream::once(async {"] # [doc = "     futures::pending!();"] # [doc = "     42_u8"] # [doc = " })));"] # [doc = " assert_eq!(p.next().await, Some(Poll::Pending));"] # [doc = " assert_eq!(p.next().await, Some(Poll::Ready(42)));"] # [doc = " assert_eq!(p.next().await, None);"] # [doc = " # });"] # [doc = " ```"] pub fn poll_immediate < S : Stream > (s : S) -> PollImmediate < S > { super :: assert_stream :: < Poll < S :: Item > , PollImmediate < S > > (PollImmediate { stream : Some (s) }) }
};
}
