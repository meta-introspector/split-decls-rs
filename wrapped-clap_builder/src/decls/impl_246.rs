macro_rules! deps {
    () => {
        ValueHint!();
        ArgExt!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [cfg (feature = "unstable-ext")] impl crate :: builder :: ArgExt for ValueHint { }
    };
}

impl_246!()