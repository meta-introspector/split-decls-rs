macro_rules! impl_548 {
    () => {
        impl < St , Fut , F > fmt :: Debug for ForEachConcurrent < St , Fut , F > where St : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ForEachConcurrent") . field ("stream" , & self . stream) . field ("futures" , & self . futures) . field ("limit" , & self . limit) . finish () } }
    };
}

impl_548!();