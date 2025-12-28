macro_rules! deps {
    () => {
        RemoteCallbacks!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl < 'a > Default for RemoteCallbacks < 'a > { fn default () -> Self { Self :: new () } }
    };
}

impl_675!();