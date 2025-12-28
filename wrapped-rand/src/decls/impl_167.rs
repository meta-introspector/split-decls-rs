macro_rules! deps {
    () => {
        SampleUniform!();
        Uniform!();
        Error!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < X : SampleUniform > TryFrom < Range < X > > for Uniform < X > { type Error = Error ; fn try_from (r : Range < X >) -> Result < Uniform < X > , Error > { Uniform :: new (r . start , r . end) } }
    };
}

impl_167!()