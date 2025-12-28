macro_rules! deps {
    () => {
        PriorityQueue!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < K : Ord + std :: fmt :: Debug , T : std :: fmt :: Debug > std :: fmt :: Debug for PriorityQueue < K , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_28!()