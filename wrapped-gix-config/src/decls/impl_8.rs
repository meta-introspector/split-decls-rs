macro_rules! deps {
    () => {
        Body!();
        Event!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'event > file :: section :: Body < 'event > { pub (crate) fn as_mut (& mut self) -> & mut Vec < Event < 'event > > { & mut self . 0 } }
    };
}

impl_8!()