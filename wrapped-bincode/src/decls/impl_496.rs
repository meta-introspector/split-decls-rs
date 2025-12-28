macro_rules! deps {
    () => {
        Encoder!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < T > Encoder for & mut T where T : Encoder , { type W = T :: W ; type C = T :: C ; fn writer (& mut self) -> & mut Self :: W { T :: writer (self) } fn config (& self) -> & Self :: C { T :: config (self) } }
    };
}

impl_496!()