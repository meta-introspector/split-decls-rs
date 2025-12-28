macro_rules! deps {
    () => {
        ChainState!();
    };
}

macro_rules! Chain {
    () => {
        deps!();
        # [doc = " An iterator which yields the elements of one iterator followed by another."] # [derive (Clone , Debug)] pub struct Chain < T , U > { front : T , back : U , state : ChainState , }
    };
}

Chain!();