macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : Ord + std :: fmt :: Debug , T : std :: fmt :: Debug > std :: fmt :: Debug for Item < K , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "({:?}: {:?})" , self . key , self . value) } }
    };
}

impl_27!()