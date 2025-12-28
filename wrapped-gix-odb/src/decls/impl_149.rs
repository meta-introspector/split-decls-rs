macro_rules! deps {
    () => {
        Cache!();
        Handle!();
        Store!();
        Proxy!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > { # [doc = " No op, as we are containing an arc handle already."] pub fn into_arc (self) -> std :: io :: Result < Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > > { Ok (self) } }
    };
}

impl_149!();