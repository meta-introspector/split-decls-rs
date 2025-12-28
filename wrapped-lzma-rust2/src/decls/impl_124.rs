macro_rules! deps {
    () => {
        WorkStealingQueue!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < T > Default for WorkStealingQueue < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_124!();