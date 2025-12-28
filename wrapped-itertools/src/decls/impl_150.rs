macro_rules! deps {
    () => {
        EitherOrBoth!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < A , B > From < EitherOrBoth < A , B > > for Option < Either < A , B > > { fn from (value : EitherOrBoth < A , B >) -> Self { match value { Left (l) => Some (Either :: Left (l)) , Right (r) => Some (Either :: Right (r)) , Both (..) => None , } } }
    };
}

impl_150!();