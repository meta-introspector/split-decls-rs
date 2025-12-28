macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < T : Eq + Copy > Eq for Cell < T > { }
    };
}

impl_207!()