macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for BigEndian { fn default () -> BigEndian { panic ! ("BigEndian default") } }
    };
}

impl_10!()