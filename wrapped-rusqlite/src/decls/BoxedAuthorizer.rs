macro_rules! deps {
    () => {
        Authorization!();
        AuthContext!();
    };
}

macro_rules! BoxedAuthorizer {
    () => {
        deps!();
        pub (crate) type BoxedAuthorizer = Box < dyn for < 'c > FnMut (AuthContext < 'c >) -> Authorization + Send + 'static > ;
    };
}

BoxedAuthorizer!();