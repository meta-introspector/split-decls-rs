macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < A , B , T > Either < (A , T) , (B , T) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the second element of the pairs."] pub fn factor_second (self) -> (Either < A , B > , T) { match self { Self :: Left ((a , x)) => (Either :: Left (a) , x) , Self :: Right ((b , x)) => (Either :: Right (b) , x) , } } }
    };
}

impl_268!();