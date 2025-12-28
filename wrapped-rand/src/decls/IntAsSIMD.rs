macro_rules! IntAsSIMD {
    () => {
        pub (crate) trait IntAsSIMD : Sized { # [inline (always)] fn splat (scalar : Self) -> Self { scalar } }
    };
}

IntAsSIMD!()