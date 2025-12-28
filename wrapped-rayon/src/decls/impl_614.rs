macro_rules! deps {
    () => {
        Inspect!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < I , F > Inspect < I , F > { # [doc = " Creates a new `Inspect` iterator."] pub (super) fn new (base : I , inspect_op : F) -> Self { Inspect { base , inspect_op } } }
    };
}

impl_614!();