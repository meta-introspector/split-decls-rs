macro_rules! MapSpecialCaseFnInto {
    () => {
        pub struct MapSpecialCaseFnInto < U > (PhantomData < U >) ;
    };
}

MapSpecialCaseFnInto!();