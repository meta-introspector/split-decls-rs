macro_rules! deps {
    () => {
        Registry!();
        Poll!();
        Token!();
        Waker!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Waker { # [doc = " Create a new `Waker`."] pub fn new (registry : & Registry , token : Token) -> io :: Result < Waker > { # [cfg (debug_assertions)] registry . register_waker () ; sys :: Waker :: new (registry . selector () , token) . map (| inner | Waker { inner }) } # [doc = " Wake up the [`Poll`] associated with this `Waker`."] # [doc = ""] # [doc = " [`Poll`]: struct.Poll.html"] pub fn wake (& self) -> io :: Result < () > { self . inner . wake () } }
    };
}

impl_47!()