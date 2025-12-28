macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl < St , Fut > TakeUntil < St , Fut > where St : Stream , Fut : Future , { pub (super) fn new (stream : St , fut : Fut) -> Self { Self { stream , fut : Some (fut) , fut_result : None , free : false } } delegate_access_inner ! (stream , St , ()) ; # [doc = " Extract the stopping future out of the combinator."] # [doc = " The future is returned only if it isn't resolved yet, ie. if the stream isn't stopped yet."] # [doc = " Taking out the future means the combinator will be yielding"] # [doc = " elements from the wrapped stream without ever stopping it."] pub fn take_future (& mut self) -> Option < Fut > { if self . fut . is_some () { self . free = true ; } self . fut . take () } # [doc = " Once the stopping future is resolved, this method can be used"] # [doc = " to extract the value returned by the stopping future."] # [doc = ""] # [doc = " This may be used to retrieve arbitrary data from the stopping"] # [doc = " future, for example a reason why the stream was stopped."] # [doc = ""] # [doc = " This method will return `None` if the future isn't resolved yet,"] # [doc = " or if the result was already taken out."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = " use futures::stream::{self, StreamExt};"] # [doc = " use futures::task::Poll;"] # [doc = ""] # [doc = " let stream = stream::iter(1..=10);"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let stop_fut = future::poll_fn(|_cx| {"] # [doc = "     i += 1;"] # [doc = "     if i <= 5 {"] # [doc = "         Poll::Pending"] # [doc = "     } else {"] # [doc = "         Poll::Ready(\"reason\")"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " let mut stream = stream.take_until(stop_fut);"] # [doc = " let _ = stream.by_ref().collect::<Vec<_>>().await;"] # [doc = ""] # [doc = " let result = stream.take_result().unwrap();"] # [doc = " assert_eq!(result, \"reason\");"] # [doc = " # });"] # [doc = " ```"] pub fn take_result (& mut self) -> Option < Fut :: Output > { self . fut_result . take () } # [doc = " Whether the stream was stopped yet by the stopping future"] # [doc = " being resolved."] pub fn is_stopped (& self) -> bool { ! self . free && self . fut . is_none () } }
    };
}

impl_452!()