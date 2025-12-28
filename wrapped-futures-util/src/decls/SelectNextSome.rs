macro_rules! SelectNextSome {
    () => {
        # [doc = " Future for the [`select_next_some`](super::StreamExt::select_next_some)"] # [doc = " method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SelectNextSome < 'a , St : ? Sized > { stream : & 'a mut St , }
    };
}

SelectNextSome!();