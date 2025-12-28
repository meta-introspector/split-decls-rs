macro_rules! deps {
    () => {
        DebugPrint!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T > Debug for DebugPrint < '_ , T > where T : Debug { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Debug :: fmt (self . 0 , f) } }
    };
}

impl_14!()