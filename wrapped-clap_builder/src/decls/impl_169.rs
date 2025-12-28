macro_rules! deps {
    () => {
        Resettable!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T > From < Option < T > > for Resettable < T > { fn from (other : Option < T >) -> Self { match other { Some (inner) => Self :: Value (inner) , None => Self :: Reset , } } }
    };
}

impl_169!();