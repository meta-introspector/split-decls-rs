macro_rules! deps {
    () => {
        Folder!();
        ReduceFolder!();
    };
}

macro_rules! impl_793 {
    () => {
        deps!();
        impl < 'r , R , T > Folder < T > for ReduceFolder < 'r , R , T > where R : Fn (T , T) -> T , { type Result = T ; fn consume (self , item : T) -> Self { ReduceFolder { reduce_op : self . reduce_op , item : (self . reduce_op) (self . item , item) , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { ReduceFolder { reduce_op : self . reduce_op , item : iter . into_iter () . fold (self . item , self . reduce_op) , } } fn complete (self) -> T { self . item } fn full (& self) -> bool { false } }
    };
}

impl_793!()