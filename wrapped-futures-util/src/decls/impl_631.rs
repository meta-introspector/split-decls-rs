macro_rules! impl_631 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryFilterMap < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFilterMap") . field ("stream" , & self . stream) . field ("pending" , & self . pending) . finish () } }
    };
}

impl_631!()