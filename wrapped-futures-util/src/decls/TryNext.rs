macro_rules! TryNext {
    () => {
        # [doc = " Future for the [`try_next`](super::TryStreamExt::try_next) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryNext < 'a , St : ? Sized > { stream : & 'a mut St , }
    };
}

TryNext!()