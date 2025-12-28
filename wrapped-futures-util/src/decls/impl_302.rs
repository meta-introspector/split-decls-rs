macro_rules! impl_302 {
    () => {
        impl < St > fmt :: Debug for Count < St > where St : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Count") . field ("stream" , & self . stream) . field ("count" , & self . count) . finish () } }
    };
}

impl_302!()