macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > HistoryBufInner < T , S > { # [doc = " Clears the buffer"] pub fn clear (& mut self) { unsafe { self . drop_contents () } ; self . write_at = 0 ; self . filled = false ; } }
    };
}

impl_62!()