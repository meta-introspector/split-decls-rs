macro_rules! deps {
    () => {
        PushOptions!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < 'cb > Default for PushOptions < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_656!()