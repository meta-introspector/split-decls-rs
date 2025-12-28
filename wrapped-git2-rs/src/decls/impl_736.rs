macro_rules! deps {
    () => {
        StashApplyOptions!();
    };
}

macro_rules! impl_736 {
    () => {
        deps!();
        impl < 'cb > Default for StashApplyOptions < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_736!();