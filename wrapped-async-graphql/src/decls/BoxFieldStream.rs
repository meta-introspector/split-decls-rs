macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! BoxFieldStream {
    () => {
        deps!();
        pub (crate) type BoxFieldStream < 'a > = Pin < Box < dyn Stream < Item = Response > + 'a + Send > > ;
    };
}

BoxFieldStream!()