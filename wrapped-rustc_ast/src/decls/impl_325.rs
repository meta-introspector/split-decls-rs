macro_rules! deps {
    () => {
        AutoDiffItem!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl fmt :: Display for AutoDiffItem { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Differentiating {} -> {}" , self . source , self . target) ? ; write ! (f , " with attributes: {:?}" , self . attrs) } }
    };
}

impl_325!()