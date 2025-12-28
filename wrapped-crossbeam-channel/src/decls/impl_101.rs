macro_rules! deps {
    () => {
        ArrayToken!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Default for ArrayToken { # [inline] fn default () -> Self { Self { slot : ptr :: null () , stamp : 0 , } } }
    };
}

impl_101!()