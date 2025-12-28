macro_rules! deps {
    () => {
        EdgeType!();
        GraphMap!();
    };
}

macro_rules! impl_877 {
    () => {
        deps!();
        impl < N : Eq + Hash + fmt :: Debug , E : fmt :: Debug , Ty : EdgeType , S : BuildHasher > fmt :: Debug for GraphMap < N , E , Ty , S > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . nodes . fmt (f) } }
    };
}

impl_877!();