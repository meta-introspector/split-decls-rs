macro_rules! deps {
    () => {
        Map!();
        IntoFallibleIterator!();
    };
}

macro_rules! FlatMap {
    () => {
        deps!();
        # [doc = " An iterator which maps each element to another iterator, yielding those iterator's elements."] # [derive (Clone , Debug)] pub struct FlatMap < I , U , F > where U : IntoFallibleIterator , { it : Map < I , F > , cur : Option < U :: IntoFallibleIter > , }
    };
}

FlatMap!()