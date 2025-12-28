macro_rules! deps {
    () => {
        GeneratorObj!();
    };
}

macro_rules! Generator {
    () => {
        deps!();
        # [doc = " the generator type, the functor passed to it must be Send"] pub type Generator < 'a , A , T > = GeneratorObj < 'a , A , T , false > ;
    };
}

Generator!()