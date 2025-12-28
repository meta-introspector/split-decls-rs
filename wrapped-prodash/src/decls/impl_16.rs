macro_rules! deps {
    () => {
        Root!();
        Options!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl From < Options > for Arc < Root > { fn from (opts : Options) -> Self { Arc :: new (opts . into ()) } }
    };
}

impl_16!();