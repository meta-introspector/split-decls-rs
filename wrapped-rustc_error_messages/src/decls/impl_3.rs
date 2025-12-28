macro_rules! deps {
    () => {
        DiagArgFromDisplay!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'a , T : fmt :: Display > From < & 'a T > for DiagArgFromDisplay < 'a > { fn from (t : & 'a T) -> Self { DiagArgFromDisplay (t) } }
    };
}

impl_3!()