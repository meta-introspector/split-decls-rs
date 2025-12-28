macro_rules! deps {
    () => {
        Operand!();
        Rvalue!();
    };
}

macro_rules! impl_894 {
    () => {
        deps!();
        impl < 'db > From < Operand < 'db > > for Rvalue < 'db > { fn from (x : Operand < 'db >) -> Self { Self :: Use (x) } }
    };
}

impl_894!()