macro_rules! deps {
    () => {
        Pat!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl From < Box < Pat > > for Pat { fn from (value : Box < Pat >) -> Self { * value } }
    };
}

impl_52!();