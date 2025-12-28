macro_rules! deps {
    () => {
        Handle!();
        Cache!();
        Store!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Cache < crate :: store :: Handle < Arc < crate :: Store > > > { # [doc = " No op, as we are containing an arc handle already."] pub fn into_arc (self) -> std :: io :: Result < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > { Ok (self) } }
    };
}

impl_137!();