macro_rules! deps {
    () => {
        VarianceTerm!();
    };
}

macro_rules! VarianceTermPtr {
    () => {
        deps!();
        pub (crate) type VarianceTermPtr < 'a > = & 'a VarianceTerm < 'a > ;
    };
}

VarianceTermPtr!()