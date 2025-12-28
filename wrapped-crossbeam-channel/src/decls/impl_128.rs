macro_rules! deps {
    () => {
        ListToken!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Default for ListToken { # [inline] fn default () -> Self { Self { block : ptr :: null () , offset : 0 , } } }
    };
}

impl_128!()