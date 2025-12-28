macro_rules! deps {
    () => {
        Sink!();
        CompatSink!();
    };
}

macro_rules! impl_1032 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < T , Item > CompatSink < T , Item > { # [doc = " Creates a new [`CompatSink`]."] pub fn new (inner : T) -> Self { Self { inner , _phantom : PhantomData } } # [doc = " Get a reference to 0.3 Sink contained within."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to 0.3 Sink contained within."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Returns the inner item."] pub fn into_inner (self) -> T { self . inner } }
    };
}

impl_1032!();