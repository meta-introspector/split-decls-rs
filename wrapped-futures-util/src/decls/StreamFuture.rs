macro_rules! StreamFuture {
    () => {
        # [doc = " Future for the [`into_future`](super::StreamExt::into_future) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct StreamFuture < St > { stream : Option < St > , }
    };
}

StreamFuture!();