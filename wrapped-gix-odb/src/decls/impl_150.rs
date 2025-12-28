macro_rules! deps {
    () => {
        Store!();
        Proxy!();
        Handle!();
        Cache!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Proxy < Cache < crate :: store :: Handle < Rc < crate :: Store > > > > { # [doc = " Create an entirely new instance, but with the in-memory objects moving between them."] pub fn into_arc (self) -> std :: io :: Result < Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > > { Ok (Proxy { inner : self . inner . into_arc () ? , object_hash : self . object_hash , memory : self . memory , }) } }
    };
}

impl_150!()