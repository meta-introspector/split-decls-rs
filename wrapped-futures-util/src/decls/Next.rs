macro_rules! Next {
    () => {
        # [doc = " Future for the [`next`](super::StreamExt::next) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Next < 'a , St : ? Sized > { stream : & 'a mut St , }
    };
}

Next!();