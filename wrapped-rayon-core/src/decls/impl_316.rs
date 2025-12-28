macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl ThreadPoolBuilder { # [doc = " Creates and returns a valid rayon thread pool builder, but does not initialize it."] pub fn new () -> Self { Self :: default () } }
    };
}

impl_316!();