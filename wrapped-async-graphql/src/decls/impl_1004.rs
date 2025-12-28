macro_rules! deps {
    () => {
        CacheControl!();
    };
}

macro_rules! impl_1004 {
    () => {
        deps!();
        impl Default for CacheControl { fn default () -> Self { Self { public : true , max_age : 0 , } } }
    };
}

impl_1004!();