macro_rules! deps {
    () => {
        PkeParameters!();
    };
}

macro_rules! KeyPairInternal {
    () => {
        deps!();
        pub (crate) struct KeyPairInternal < Pke : PkeParameters > { _phantom : PhantomData < Pke > , }
    };
}

KeyPairInternal!();