macro_rules! deps {
    () => {
        StripStream!();
        Stderr!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl StripStream < std :: io :: Stderr > { # [doc = " Get exclusive access to the `StripStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> StripStream < std :: io :: StderrLock < 'static > > { StripStream { raw : self . raw . lock () , state : self . state , } } }
    };
}

impl_116!();