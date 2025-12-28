macro_rules! deps {
    () => {
        ReadRef!();
        Segment!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > read :: private :: Sealed for Segment < 'data , 'file , R > { }
    };
}

impl_133!();