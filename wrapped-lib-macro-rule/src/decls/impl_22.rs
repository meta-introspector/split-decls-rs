macro_rules! deps {
    () => {
        Failure!();
        ErrParse!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T > ErrParse < T > { pub fn new (failures : Vec < Failure >) -> Self { ErrParse { failures , _phantom : std :: marker :: PhantomData } } }
    };
}

impl_22!()