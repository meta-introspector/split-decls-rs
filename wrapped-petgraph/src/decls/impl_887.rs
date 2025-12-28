macro_rules! deps {
    () => {
        GraphMap!();
        Create!();
    };
}

macro_rules! impl_887 {
    () => {
        deps!();
        impl < N , E , Ty , S > GraphMap < N , E , Ty , S > where S : BuildHasher , { # [doc = " Create a new `GraphMap`"] pub fn new () -> Self where S : Default , { Self :: default () } # [doc = " Create a new `GraphMap` with estimated capacity."] pub fn with_capacity (nodes : usize , edges : usize) -> Self where S : Default , { Self { nodes : IndexMap :: with_capacity_and_hasher (nodes , S :: default ()) , edges : IndexMap :: with_capacity_and_hasher (edges , S :: default ()) , ty : PhantomData , } } # [doc = " Create a new `GraphMap` with estimated capacity, and specified hasher."] pub fn with_capacity_and_hasher (nodes : usize , edges : usize , hasher : S) -> Self where S : Clone , { Self { nodes : IndexMap :: with_capacity_and_hasher (nodes , hasher . clone ()) , edges : IndexMap :: with_capacity_and_hasher (edges , hasher) , ty : PhantomData , } } }
    };
}

impl_887!();