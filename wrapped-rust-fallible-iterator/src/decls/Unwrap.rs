macro_rules! deps {
    () => {
        FallibleIterator!();
    };
}

macro_rules! Unwrap {
    () => {
        deps!();
        # [doc = " An iterator that unwraps every element yielded by the underlying"] # [doc = " FallibleIterator"] # [derive (Clone , Debug)] pub struct Unwrap < T > (T) ;
    };
}

Unwrap!();