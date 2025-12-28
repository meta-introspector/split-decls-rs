macro_rules! deps {
    () => {
        Stderr!();
        WinconStream!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl WinconStream < std :: io :: Stderr > { # [doc = " Get exclusive access to the `WinconStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> WinconStream < std :: io :: StderrLock < 'static > > { WinconStream { raw : self . raw . lock () , state : self . state , } } }
    };
}

impl_128!();