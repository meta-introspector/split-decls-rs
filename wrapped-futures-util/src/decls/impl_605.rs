macro_rules! impl_605 {
    () => {
        impl < St , Fut , F > fmt :: Debug for OrElse < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OrElse") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
    };
}

impl_605!();