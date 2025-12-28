macro_rules! deps {
    () => {
        Arena!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > FromIterator < T > for Arena < T > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { Arena { data : Vec :: from_iter (iter) } } }
    };
}

impl_35!()