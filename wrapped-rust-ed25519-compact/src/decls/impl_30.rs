macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Eq for Fe { }
    };
}

impl_30!();