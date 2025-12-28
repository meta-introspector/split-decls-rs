macro_rules! deps {
    () => {
        Lifetime!();
        ParamKindOrd!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl fmt :: Display for ParamKindOrd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ParamKindOrd :: Lifetime => "lifetime" . fmt (f) , ParamKindOrd :: TypeOrConst => "type and const" . fmt (f) , } } }
    };
}

impl_33!()