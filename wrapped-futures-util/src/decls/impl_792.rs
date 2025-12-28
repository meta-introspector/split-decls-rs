macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_792 {
    () => {
        deps!();
        impl < St1 , St2 > Select < St1 , St2 > { # [doc = " Acquires a reference to the underlying streams that this combinator is"] # [doc = " pulling from."] pub fn get_ref (& self) -> (& St1 , & St2) { self . inner . get_ref () } # [doc = " Acquires a mutable reference to the underlying streams that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " stream which may otherwise confuse this combinator."] pub fn get_mut (& mut self) -> (& mut St1 , & mut St2) { self . inner . get_mut () } # [doc = " Acquires a pinned mutable reference to the underlying streams that this"] # [doc = " combinator is pulling from."] # [doc = ""] # [doc = " Note that care must be taken to avoid tampering with the state of the"] # [doc = " stream which may otherwise confuse this combinator."] pub fn get_pin_mut (self : Pin < & mut Self >) -> (Pin < & mut St1 > , Pin < & mut St2 >) { let this = self . project () ; this . inner . get_pin_mut () } # [doc = " Consumes this combinator, returning the underlying streams."] # [doc = ""] # [doc = " Note that this may discard intermediate state of this combinator, so"] # [doc = " care should be taken to avoid losing resources when this is called."] pub fn into_inner (self) -> (St1 , St2) { self . inner . into_inner () } }
    };
}

impl_792!()