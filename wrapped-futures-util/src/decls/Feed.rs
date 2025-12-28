macro_rules! Feed {
    () => {
        # [doc = " Future for the [`feed`](super::SinkExt::feed) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Feed < 'a , Si : ? Sized , Item > { sink : & 'a mut Si , item : Option < Item > , }
    };
}

Feed!();