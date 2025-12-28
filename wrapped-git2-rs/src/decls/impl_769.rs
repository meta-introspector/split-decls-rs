macro_rules! deps {
    () => {
        SubmoduleUpdateOptions!();
    };
}

macro_rules! impl_769 {
    () => {
        deps!();
        impl < 'cb > Default for SubmoduleUpdateOptions < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_769!();