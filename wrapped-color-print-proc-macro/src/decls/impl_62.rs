macro_rules! deps {
    () => {
        Intensity!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Intensity { pub fn new (is_bright : bool) -> Self { if is_bright { Self :: Bright } else { Self :: Normal } } }
    };
}

impl_62!();