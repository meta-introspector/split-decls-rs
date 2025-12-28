macro_rules! deps {
    () => {
        Graph!();
        Create!();
    };
}

macro_rules! impl_685 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Graph < N , E , Ty , Ix > { # [doc = " Create a new `Graph` with estimated capacity."] pub fn with_capacity (nodes : usize , edges : usize) -> Self { Graph { nodes : Vec :: with_capacity (nodes) , edges : Vec :: with_capacity (edges) , ty : PhantomData , } } }
    };
}

impl_685!()