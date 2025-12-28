macro_rules! deps {
    () => {
        VecConsumer!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a , Fut : Future > VecConsumer < 'a , Fut > { pub (crate) fn new (output : & 'a mut Vec < Fut :: Output >) -> Self { Self { group : FuturesUnordered :: new () , output , } } }
    };
}

impl_148!()