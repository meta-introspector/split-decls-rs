macro_rules! TryForEachFuture {
    () => {
        # [doc = " Future for the [`StreamExt::try_for_each()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEachFuture < 'a , S : ? Sized , F > { stream : & 'a mut S , f : F , }
    };
}

TryForEachFuture!()