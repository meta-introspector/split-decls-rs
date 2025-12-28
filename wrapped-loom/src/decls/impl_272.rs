macro_rules! deps {
    () => {
        AtomicPtr!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < T > Default for AtomicPtr < T > { fn default () -> AtomicPtr < T > { use std :: ptr ; AtomicPtr :: new (ptr :: null_mut ()) } }
    };
}

impl_272!()