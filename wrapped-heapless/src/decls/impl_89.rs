macro_rules! deps {
    () => {
        Pos!();
        Vec!();
        CoreMap!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < K , V , const N : usize > CoreMap < K , V , N > { const fn new () -> Self { const INIT : Option < Pos > = None ; Self { entries : Vec :: new () , indices : [INIT ; N] , } } }
    };
}

impl_89!()