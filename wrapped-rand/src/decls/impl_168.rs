macro_rules! deps {
    () => {
        SampleUniform!();
        Error!();
        Uniform!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < X : SampleUniform > TryFrom < RangeInclusive < X > > for Uniform < X > { type Error = Error ; fn try_from (r : :: core :: ops :: RangeInclusive < X >) -> Result < Uniform < X > , Error > { Uniform :: new_inclusive (r . start () , r . end ()) } }
    };
}

impl_168!();