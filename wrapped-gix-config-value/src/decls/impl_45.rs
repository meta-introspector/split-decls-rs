macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , BStr > > for Path < 'a > { fn from (value : Cow < 'a , BStr >) -> Self { Path { value } } }
    };
}

impl_45!();