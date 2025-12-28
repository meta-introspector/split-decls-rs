macro_rules! deps {
    () => {
        SCx!();
    };
}

macro_rules! GenericCx {
    () => {
        deps!();
        pub (crate) struct GenericCx < 'll , T : Borrow < SCx < 'll > > > (T , PhantomData < SCx < 'll > >) ;
    };
}

GenericCx!();