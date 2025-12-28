macro_rules! impl_727 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryAll < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryAll") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
    };
}

impl_727!();