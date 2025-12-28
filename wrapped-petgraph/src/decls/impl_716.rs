macro_rules! deps {
    () => {
        Graph!();
        Create!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        # [doc = " Create a new empty `Graph`."] impl < N , E , Ty , Ix > Default for Graph < N , E , Ty , Ix > { fn default () -> Self { Self :: with_capacity (0 , 0) } }
    };
}

impl_716!()