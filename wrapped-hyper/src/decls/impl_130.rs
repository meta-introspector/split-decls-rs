macro_rules! deps {
    () => {
        ReasonPhrase!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl From < ReasonPhrase > for Bytes { fn from (reason : ReasonPhrase) -> Self { reason . 0 } }
    };
}

impl_130!()