macro_rules! deps {
    () => {
        GenericDefaults!();
        EarlyBinder!();
    };
}

macro_rules! impl_846 {
    () => {
        deps!();
        impl < 'db > GenericDefaults < 'db > { # [inline] pub fn get (& self , idx : usize) -> Option < EarlyBinder < 'db , GenericArg < 'db > > > { self . 0 . as_ref () ? [idx] } }
    };
}

impl_846!();