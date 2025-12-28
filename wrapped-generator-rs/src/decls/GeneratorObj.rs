macro_rules! deps {
    () => {
        GeneratorImpl!();
        StackBox!();
    };
}

macro_rules! GeneratorObj {
    () => {
        deps!();
        # [doc = " the generator obj type, the functor passed to it must be Send"] pub struct GeneratorObj < 'a , A , T , const LOCAL : bool > { gen : StackBox < GeneratorImpl < 'a , A , T > > , }
    };
}

GeneratorObj!();