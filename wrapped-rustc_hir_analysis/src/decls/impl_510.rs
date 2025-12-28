macro_rules! deps {
    () => {
        VarianceTerm!();
        InferredIndex!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for VarianceTerm < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ConstantTerm (c1) => write ! (f , "{c1:?}") , TransformTerm (v1 , v2) => write ! (f , "({v1:?} \u{00D7} {v2:?})") , InferredTerm (id) => write ! (f , "[{}]" , { let InferredIndex (i) = id ; i }) , } } }
    };
}

impl_510!()