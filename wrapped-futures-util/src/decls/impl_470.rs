macro_rules! impl_470 {
    () => {
        impl < St , Fut , T , F > fmt :: Debug for TryFold < St , Fut , T , F > where St : fmt :: Debug , Fut : fmt :: Debug , T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFold") . field ("stream" , & self . stream) . field ("accum" , & self . accum) . field ("future" , & self . future) . finish () } }
    };
}

impl_470!();