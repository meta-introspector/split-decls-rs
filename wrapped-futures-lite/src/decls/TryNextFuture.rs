macro_rules! TryNextFuture {
    () => {
        # [doc = " Future for the [`StreamExt::try_next()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryNextFuture < 'a , S : ? Sized > { stream : & 'a mut S , }
    };
}

TryNextFuture!();