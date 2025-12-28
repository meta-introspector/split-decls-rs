macro_rules! deps {
    () => {
        Create!();
        GraphMap!();
    };
}

macro_rules! impl_908 {
    () => {
        deps!();
        # [doc = " Create a new empty `GraphMap`."] impl < N , E , Ty , S > Default for GraphMap < N , E , Ty , S > where S : BuildHasher + Default , { fn default () -> Self { GraphMap :: with_capacity (0 , 0) } }
    };
}

impl_908!()