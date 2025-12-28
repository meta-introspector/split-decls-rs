macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        # [doc = " A [Stream](crate::stream::Stream) implementation that can be polled repeatedly until the future is done."] # [doc = " The stream will never return [Poll::Pending](core::task::Poll::Pending)"] # [doc = " so polling it in a tight loop is worse than using a blocking synchronous function."] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::task::Poll;"] # [doc = " use futures::{StreamExt, future};"] # [doc = " use future::FusedFuture;"] # [doc = ""] # [doc = " let f = async { 1_u32 };"] # [doc = " let f = pin!(f);"] # [doc = " let mut r = future::poll_immediate(f);"] # [doc = " assert_eq!(r.next().await, Some(Poll::Ready(1)));"] # [doc = ""] # [doc = " let f = async {futures::pending!(); 42_u8};"] # [doc = " let f = pin!(f);"] # [doc = " let mut p = future::poll_immediate(f);"] # [doc = " assert_eq!(p.next().await, Some(Poll::Pending));"] # [doc = " assert!(!p.is_terminated());"] # [doc = " assert_eq!(p.next().await, Some(Poll::Ready(42)));"] # [doc = " assert!(p.is_terminated());"] # [doc = " assert_eq!(p.next().await, None);"] # [doc = " # });"] # [doc = " ```"] impl < T , F > Stream for PollImmediate < F > where F : Future < Output = T > , { type Item = Poll < T > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; match this . future . as_mut () . as_pin_mut () { None => Poll :: Ready (None) , Some (fut) => Poll :: Ready (Some (fut . poll (cx) . map (| t | { this . future . set (None) ; t }))) , } } }
    };
}

impl_188!();