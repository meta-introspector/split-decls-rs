macro_rules! deps {
    () => {
        SourceRef!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl std :: fmt :: Display for SourceRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { SourceRef :: FullName (name) => name . fmt (f) , SourceRef :: ObjectId (id) => id . fmt (f) , } } }
    };
}

impl_26!()