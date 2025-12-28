macro_rules! deps {
    () => {
        EitherOrBoth!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < A , B > From < Either < A , B > > for EitherOrBoth < A , B > { fn from (either : Either < A , B >) -> Self { match either { Either :: Left (l) => Left (l) , Either :: Right (l) => Right (l) , } } }
    };
}

impl_151!();