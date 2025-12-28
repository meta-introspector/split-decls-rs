macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < A , B , T > Either < (T , A) , (T , B) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the first element of the pairs."] pub fn factor_first (self) -> (T , Either < A , B >) { match self { Self :: Left ((x , a)) => (x , Either :: Left (a)) , Self :: Right ((x , b)) => (x , Either :: Right (b)) , } } }
    };
}

impl_267!();