macro_rules! deps {
    () => {
        DemangleStyle!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for DemangleStyle < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { DemangleStyle :: Legacy (ref d) => fmt :: Display :: fmt (d , f) , DemangleStyle :: V0 (ref d) => fmt :: Display :: fmt (d , f) , } } }
    };
}

impl_42!();