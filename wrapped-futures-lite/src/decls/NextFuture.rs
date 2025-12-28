macro_rules! NextFuture {
    () => {
        # [doc = " Future for the [`StreamExt::next()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct NextFuture < 'a , S : ? Sized > { stream : & 'a mut S , }
    };
}

NextFuture!()