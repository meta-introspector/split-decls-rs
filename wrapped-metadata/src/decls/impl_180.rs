macro_rules! deps {
    () => {
        Blobs!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Default for Blobs { fn default () -> Self { Self { map : Default :: default () , stream : vec ! [0] , } } }
    };
}

impl_180!();