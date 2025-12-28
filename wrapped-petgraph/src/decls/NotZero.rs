macro_rules! NotZero {
    () => {
        # [doc = " `NotZero` is used to optimize the memory usage of edge weights `E` in a"] # [doc = " [`MatrixGraph`](struct.MatrixGraph.html), replacing the default `Option<E>` sentinel."] # [doc = ""] # [doc = " Pre-requisite: edge weight should implement [`Zero`](trait.Zero.html)."] # [doc = ""] # [doc = " Note that if you're already using the standard non-zero types (such as `NonZeroU32`), you don't"] # [doc = " have to use this wrapper and can leave the default `Null` type argument."] pub struct NotZero < T > (T) ;
    };
}

NotZero!()