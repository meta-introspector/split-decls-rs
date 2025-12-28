macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < A , B > Future for Either < A , B > where A : Future , B : Future < Output = A :: Output > , { type Output = A :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . as_pin_mut () { Either :: Left (x) => x . poll (cx) , Either :: Right (x) => x . poll (cx) , } } }
    };
}

impl_270!()