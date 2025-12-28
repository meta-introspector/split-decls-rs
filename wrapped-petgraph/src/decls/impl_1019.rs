macro_rules! deps {
    () => {
        IndexType!();
        Create!();
        MatrixGraph!();
        EdgeType!();
        Nullable!();
    };
}

macro_rules! impl_1019 {
    () => {
        deps!();
        # [doc = " Create a new empty `MatrixGraph`."] impl < N , E , S : BuildHasher + Default , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Default for MatrixGraph < N , E , S , Ty , Null , Ix > { fn default () -> Self { Self :: with_capacity (0) } }
    };
}

impl_1019!();