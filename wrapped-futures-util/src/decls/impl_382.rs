macro_rules! impl_382 {
    () => {
        impl < St , F > fmt :: Debug for Map < St , F > where St : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("stream" , & self . stream) . finish () } }
    };
}

impl_382!()