macro_rules! impl_291 {
    () => {
        impl < R1 : fmt :: Debug , R2 : fmt :: Debug > fmt :: Debug for Chain < R1 , R2 > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Chain") . field ("r1" , & self . first) . field ("r2" , & self . second) . finish () } }
    };
}

impl_291!();