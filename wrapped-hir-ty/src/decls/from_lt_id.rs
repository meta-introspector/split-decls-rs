macro_rules! deps {
    () => {
        Generics!();
    };
}

macro_rules! from_lt_id {
    () => {
        deps!();
        fn from_lt_id < 'a > (it : & 'a Generics ,) -> impl Fn ((LocalLifetimeParamId , & 'a LifetimeParamData)) -> (GenericParamId , GenericParamDataRef < 'a >) { move | (local_id , p) : (_ , _) | { (GenericParamId :: LifetimeParamId (LifetimeParamId { parent : it . def , local_id }) , GenericParamDataRef :: LifetimeParamData (p) ,) } }
    };
}

from_lt_id!();