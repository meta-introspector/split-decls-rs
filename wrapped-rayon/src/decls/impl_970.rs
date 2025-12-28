macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_970 {
    () => {
        deps!();
        impl < I , F > Update < I , F > { # [doc = " Creates a new `Update` iterator."] pub (super) fn new (base : I , update_op : F) -> Self { Update { base , update_op } } }
    };
}

impl_970!();