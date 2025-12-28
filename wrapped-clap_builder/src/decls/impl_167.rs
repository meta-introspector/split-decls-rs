macro_rules! deps {
    () => {
        Resettable!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T > Resettable < T > { pub (crate) fn into_option (self) -> Option < T > { match self { Self :: Value (t) => Some (t) , Self :: Reset => None , } } }
    };
}

impl_167!()