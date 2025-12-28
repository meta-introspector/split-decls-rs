macro_rules! deps {
    () => {
        Intersperse!();
        ParallelIterator!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        impl < I > Intersperse < I > where I : ParallelIterator < Item : Clone > , { # [doc = " Creates a new `Intersperse` iterator"] pub (super) fn new (base : I , item : I :: Item) -> Self { Intersperse { base , item } } }
    };
}

impl_644!();