macro_rules! impl_211 {
    () => {
        impl < Fut1 , Fut2 > fmt :: Debug for Join < Fut1 , Fut2 > where Fut1 : Future + fmt :: Debug , Fut1 :: Output : fmt :: Debug , Fut2 : Future + fmt :: Debug , Fut2 :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Join") . field ("fut1" , & self . fut1) . field ("fut2" , & self . fut2) . finish () } }
    };
}

impl_211!();