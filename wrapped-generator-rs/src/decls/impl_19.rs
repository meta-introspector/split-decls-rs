macro_rules! deps {
    () => {
        GeneratorObj!();
        Generator!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < A , T , const LOCAL : bool > fmt :: Debug for GeneratorObj < '_ , A , T , LOCAL > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Generator<{}, Output={}, Local={}> {{ ... }}" , std :: any :: type_name ::< A > () , std :: any :: type_name ::< T > () , LOCAL) } }
    };
}

impl_19!();