macro_rules! impl_587 {
    () => {
        impl < St , Fut , F > fmt :: Debug for AndThen < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AndThen") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
    };
}

impl_587!();