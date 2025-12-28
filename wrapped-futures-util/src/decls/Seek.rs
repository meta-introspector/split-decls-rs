macro_rules! Seek {
    () => {
        # [doc = " Future for the [`seek`](crate::io::AsyncSeekExt::seek) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Seek < 'a , S : ? Sized > { seek : & 'a mut S , pos : SeekFrom , }
    };
}

Seek!();