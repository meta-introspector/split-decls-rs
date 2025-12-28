macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Default for Edition { fn default () -> Self { Self ("2021" . into ()) } }
    };
}

impl_79!()