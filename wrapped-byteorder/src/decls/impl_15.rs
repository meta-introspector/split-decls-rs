macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Default for BigEndian { fn default () -> BigEndian { panic ! ("BigEndian default") } }
    };
}

impl_15!()