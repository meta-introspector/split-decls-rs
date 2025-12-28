macro_rules! deps {
    () => {
        Parser!();
        Input!();
        Result!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'a , V , F > Parser < 'a , V > for F where F : FnMut (Input < 'a >) -> Result < 'a , V > { }
    };
}

impl_90!();