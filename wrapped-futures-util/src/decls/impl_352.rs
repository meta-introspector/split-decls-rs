macro_rules! impl_352 {
    () => {
        impl < St , Fut , F > fmt :: Debug for All < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("All") . field ("stream" , & self . stream) . field ("done" , & self . done) . field ("future" , & self . future) . finish () } }
    };
}

impl_352!();