macro_rules! deps {
    () => {
        Strings!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl Default for Strings { fn default () -> Self { Self { map : Default :: default () , stream : vec ! [0] , } } }
    };
}

impl_184!()