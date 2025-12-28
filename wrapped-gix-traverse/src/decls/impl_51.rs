macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl State { fn clear (& mut self) { self . next . clear () ; self . buf . clear () ; } }
    };
}

impl_51!()