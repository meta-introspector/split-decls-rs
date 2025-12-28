macro_rules! deps {
    () => {
        HomogeneousTuple!();
    };
}

macro_rules! TupleBuffer {
    () => {
        deps!();
        # [doc = " An iterator over a incomplete tuple."] # [doc = ""] # [doc = " See [`.tuples()`](crate::Itertools::tuples) and"] # [doc = " [`Tuples::into_buffer()`]."] # [derive (Clone , Debug)] pub struct TupleBuffer < T > where T : HomogeneousTuple , { cur : usize , buf : T :: Buffer , }
    };
}

TupleBuffer!();