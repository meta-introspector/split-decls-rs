macro_rules! deps {
    () => {
        StripStream!();
        Stdout!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl StripStream < std :: io :: Stdout > { # [doc = " Get exclusive access to the `StripStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> StripStream < std :: io :: StdoutLock < 'static > > { StripStream { raw : self . raw . lock () , state : self . state , } } }
    };
}

impl_115!();