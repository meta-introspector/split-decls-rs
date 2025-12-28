macro_rules! once_future {
    () => {
        # [doc = " Creates a stream that invokes the given future as its first item, and then"] # [doc = " produces no more items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::{stream, prelude::*};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut stream = Box::pin(stream::once_future(async { 1 }));"] # [doc = " assert_eq!(stream.next().await, Some(1));"] # [doc = " assert_eq!(stream.next().await, None);"] # [doc = " # });"] # [doc = " ```"] pub fn once_future < F : Future > (future : F) -> OnceFuture < F > { OnceFuture { future : Some (future) , } }
    };
}

once_future!()