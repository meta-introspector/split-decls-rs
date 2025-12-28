macro_rules! deps {
    () => {
        ParDrainProducer!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T > Drop for ParDrainProducer < T > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { if mem :: needs_drop :: < T > () { for item in & mut self . iter { unsafe { item . drop () ; } } } } }
    };
}

impl_177!()