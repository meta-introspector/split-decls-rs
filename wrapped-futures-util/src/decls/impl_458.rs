macro_rules! impl_458 {
    () => {
        impl < St , Fut , F > fmt :: Debug for Then < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Then") . field ("stream" , & self . stream) . field ("future" , & self . future) . finish () } }
    };
}

impl_458!()