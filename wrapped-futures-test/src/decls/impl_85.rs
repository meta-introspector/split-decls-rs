macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T > InterleavePending < T > { pub (crate) fn new (inner : T) -> Self { Self { inner , pended : false } } # [doc = " Acquires a reference to the underlying I/O object that this adaptor is"] # [doc = " wrapping."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Acquires a mutable reference to the underlying I/O object that this"] # [doc = " adaptor is wrapping."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Acquires a pinned mutable reference to the underlying I/O object that"] # [doc = " this adaptor is wrapping."] pub fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut T > { self . project () . inner } # [doc = " Consumes this adaptor returning the underlying I/O object."] pub fn into_inner (self) -> T { self . inner } fn poll_with < 'a , U > (self : Pin < & 'a mut Self > , cx : & mut Context < '_ > , f : impl FnOnce (Pin < & 'a mut T > , & mut Context < '_ >) -> Poll < U > ,) -> Poll < U > { let this = self . project () ; if * this . pended { let next = f (this . inner , cx) ; if next . is_ready () { * this . pended = false ; } next } else { cx . waker () . wake_by_ref () ; * this . pended = true ; Poll :: Pending } } }
    };
}

impl_85!()