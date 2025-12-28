macro_rules! deps {
    () => {
        TrackClosed!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T > TrackClosed < T > { pub (crate) fn new (inner : T) -> Self { Self { inner , closed : false } } # [doc = " Check whether this object has been closed."] pub fn is_closed (& self) -> bool { self . closed } # [doc = " Acquires a reference to the underlying object that this adaptor is"] # [doc = " wrapping."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Acquires a mutable reference to the underlying object that this"] # [doc = " adaptor is wrapping."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Acquires a pinned mutable reference to the underlying object that"] # [doc = " this adaptor is wrapping."] pub fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut T > { self . project () . inner } # [doc = " Consumes this adaptor returning the underlying object."] pub fn into_inner (self) -> T { self . inner } }
    };
}

impl_97!()