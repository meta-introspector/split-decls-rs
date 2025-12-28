macro_rules! impl_573 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryForEachConcurrent < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryForEachConcurrent") . field ("stream" , & self . stream) . field ("futures" , & self . futures) . field ("limit" , & self . limit) . finish () } }
    };
}

impl_573!();