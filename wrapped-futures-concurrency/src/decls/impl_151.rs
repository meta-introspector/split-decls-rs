macro_rules! deps {
    () => {
        ResultVecConsumer!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'a , Fut : Future , T , E > ResultVecConsumer < 'a , Fut , T , E > { pub (crate) fn new (output : & 'a mut Result < Vec < T > , E >) -> Self { Self { group : FuturesUnordered :: new () , output , } } }
    };
}

impl_151!()