macro_rules! deps {
    () => {
        IndexType!();
        StableGraph!();
        Create!();
    };
}

macro_rules! impl_815 {
    () => {
        deps!();
        # [doc = " Create a new empty `StableGraph`."] impl < N , E , Ty , Ix > Default for StableGraph < N , E , Ty , Ix > where Ix : IndexType , { fn default () -> Self { Self :: with_capacity (0 , 0) } }
    };
}

impl_815!();