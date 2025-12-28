macro_rules! deps {
    () => {
        Slots!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Default for Slots { fn default () -> Self { Slots :: AsNeededByDiskState { multiplier : 1.1 , minimum : 32 , } } }
    };
}

impl_34!()