macro_rules! deps {
    () => {
        DiagArgFromDisplay!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'a > From < & 'a dyn fmt :: Display > for DiagArgFromDisplay < 'a > { fn from (t : & 'a dyn fmt :: Display) -> Self { DiagArgFromDisplay (t) } }
    };
}

impl_2!();