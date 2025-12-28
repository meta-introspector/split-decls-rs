macro_rules! deps {
    () => {
        IdentityWithResult!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < Input , Error > Default for IdentityWithResult < Input , Error > { fn default () -> Self { IdentityWithResult { _input : Default :: default () , _error : Default :: default () , } } }
    };
}

impl_64!()