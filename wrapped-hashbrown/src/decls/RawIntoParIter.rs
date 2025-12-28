macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! RawIntoParIter {
    () => {
        deps!();
        # [doc = " Parallel iterator which consumes a table and returns elements."] pub struct RawIntoParIter < T , A : Allocator = Global > { table : RawTable < T , A > , }
    };
}

RawIntoParIter!();