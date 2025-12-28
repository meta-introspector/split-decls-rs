macro_rules! deps {
    () => {
        OccupiedError!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > fmt :: Display for OccupiedError < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to insert {:?}, key {:?} already exists with value {:?}" , self . value , self . entry . key () , self . entry . get () ,) } }
    };
}

impl_293!()