macro_rules! impl_326 {
    () => {
        impl < St , Fut , F > fmt :: Debug for FilterMap < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FilterMap") . field ("stream" , & self . stream) . field ("pending" , & self . pending) . finish () } }
    };
}

impl_326!();