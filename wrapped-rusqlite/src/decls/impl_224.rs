macro_rules! deps {
    () => {
        Statement!();
        Row!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'stmt > AsRef < Statement < 'stmt > > for Row < 'stmt > { fn as_ref (& self) -> & Statement < 'stmt > { self . stmt } }
    };
}

impl_224!();