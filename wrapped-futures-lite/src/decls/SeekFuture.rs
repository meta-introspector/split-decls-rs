macro_rules! SeekFuture {
    () => {
        # [doc = " Future for the [`AsyncSeekExt::seek()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SeekFuture < 'a , S : Unpin + ? Sized > { seeker : & 'a mut S , pos : SeekFrom , }
    };
}

SeekFuture!()