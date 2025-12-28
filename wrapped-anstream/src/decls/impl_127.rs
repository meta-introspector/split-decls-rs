macro_rules! deps {
    () => {
        WinconStream!();
        Stdout!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl WinconStream < std :: io :: Stdout > { # [doc = " Get exclusive access to the `WinconStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> WinconStream < std :: io :: StdoutLock < 'static > > { WinconStream { raw : self . raw . lock () , state : self . state , } } }
    };
}

impl_127!()