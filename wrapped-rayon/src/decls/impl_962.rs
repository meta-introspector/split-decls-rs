macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
        Collector!();
    };
}

macro_rules! impl_962 {
    () => {
        deps!();
        impl < A , B , FromA , FromB > FromParallelIterator < (A , B) > for (FromA , FromB) where A : Send , B : Send , FromA : Send + FromParallelIterator < A > , FromB : Send + FromParallelIterator < B > , { fn from_par_iter < I > (pi : I) -> Self where I : IntoParallelIterator < Item = (A , B) > , { let (a , b) : (Collector < FromA > , Collector < FromB >) = pi . into_par_iter () . unzip () ; (a . result . unwrap () , b . result . unwrap ()) } }
    };
}

impl_962!();