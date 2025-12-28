macro_rules! deps {
    () => {
        Seek!();
    };
}

macro_rules! impl_1193 {
    () => {
        deps!();
        impl < 'a , S : AsyncSeek + ? Sized + Unpin > Seek < 'a , S > { pub (super) fn new (seek : & 'a mut S , pos : SeekFrom) -> Self { Self { seek , pos } } }
    };
}

impl_1193!();