macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Diff { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (self . hunks ()) . finish () } }
    };
}

impl_12!()