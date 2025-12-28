macro_rules! impl_465 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryForEach < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryForEach") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
    };
}

impl_465!();