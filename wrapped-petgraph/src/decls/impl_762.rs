macro_rules! deps {
    () => {
        Frozen!();
        Graph!();
        EdgeType!();
        GraphIndex!();
        IndexType!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Frozen < '_ , Graph < N , E , Ty , Ix > > where Ty : EdgeType , Ix : IndexType , { # [allow (clippy :: type_complexity)] # [doc = " Index the `Graph` by two indices, any combination of"] # [doc = " node or edge indices is fine."] # [doc = ""] # [doc = " **Panics** if the indices are equal or if they are out of bounds."] # [track_caller] pub fn index_twice_mut < T , U > (& mut self , i : T , j : U ,) -> (& mut < Graph < N , E , Ty , Ix > as Index < T > > :: Output , & mut < Graph < N , E , Ty , Ix > as Index < U > > :: Output ,) where Graph < N , E , Ty , Ix > : IndexMut < T > + IndexMut < U > , T : GraphIndex , U : GraphIndex , { self . 0 . index_twice_mut (i , j) } }
    };
}

impl_762!()