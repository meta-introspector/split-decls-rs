macro_rules! deps {
    () => {
        StreamInner!();
        AutoStream!();
        Stderr!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl AutoStream < std :: io :: Stderr > { # [doc = " Get exclusive access to the `AutoStream`"] # [doc = ""] # [doc = " Why?"] # [doc = " - Faster performance when writing in a loop"] # [doc = " - Avoid other threads interleaving output with the current thread"] # [inline] pub fn lock (self) -> AutoStream < std :: io :: StderrLock < 'static > > { let inner = match self . inner { StreamInner :: PassThrough (w) => StreamInner :: PassThrough (w . lock ()) , StreamInner :: Strip (w) => StreamInner :: Strip (w . lock ()) , # [cfg (all (windows , feature = "wincon"))] StreamInner :: Wincon (w) => StreamInner :: Wincon (w . lock ()) , } ; AutoStream { inner } } }
    };
}

impl_99!()