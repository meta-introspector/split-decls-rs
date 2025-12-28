macro_rules! deps {
    () => {
        WakerToHandle!();
        Compat01As03!();
    };
}

macro_rules! impl_1007 {
    () => {
        deps!();
        impl < T > Compat01As03 < T > { # [doc = " Wraps a futures 0.1 Future, Stream, AsyncRead, or AsyncWrite"] # [doc = " object in a futures 0.3-compatible wrapper."] pub fn new (object : T) -> Self { Self { inner : spawn01 (object) } } fn in_notify < R > (& mut self , cx : & mut Context < '_ > , f : impl FnOnce (& mut T) -> R) -> R { let notify = & WakerToHandle (cx . waker ()) ; self . inner . poll_fn_notify (notify , 0 , f) } # [doc = " Get a reference to 0.1 Future, Stream, AsyncRead, or AsyncWrite object contained within."] pub fn get_ref (& self) -> & T { self . inner . get_ref () } # [doc = " Get a mutable reference to 0.1 Future, Stream, AsyncRead or AsyncWrite object contained"] # [doc = " within."] pub fn get_mut (& mut self) -> & mut T { self . inner . get_mut () } # [doc = " Consume this wrapper to return the underlying 0.1 Future, Stream, AsyncRead, or"] # [doc = " AsyncWrite object."] pub fn into_inner (self) -> T { self . inner . into_inner () } }
    };
}

impl_1007!()