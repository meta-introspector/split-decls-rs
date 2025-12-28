macro_rules! impl_926 {
    () => {
        impl < Si1 , Si2 > Fanout < Si1 , Si2 > { pub (super) fn new (sink1 : Si1 , sink2 : Si2) -> Self { Self { sink1 , sink2 } } # [doc = " Get a shared reference to the inner sinks."] pub fn get_ref (& self) -> (& Si1 , & Si2) { (& self . sink1 , & self . sink2) } # [doc = " Get a mutable reference to the inner sinks."] pub fn get_mut (& mut self) -> (& mut Si1 , & mut Si2) { (& mut self . sink1 , & mut self . sink2) } # [doc = " Get a pinned mutable reference to the inner sinks."] pub fn get_pin_mut (self : Pin < & mut Self >) -> (Pin < & mut Si1 > , Pin < & mut Si2 >) { let this = self . project () ; (this . sink1 , this . sink2) } # [doc = " Consumes this combinator, returning the underlying sinks."] # [doc = ""] # [doc = " Note that this may discard intermediate state of this combinator,"] # [doc = " so care should be taken to avoid losing resources when this is called."] pub fn into_inner (self) -> (Si1 , Si2) { (self . sink1 , self . sink2) } }
    };
}

impl_926!()