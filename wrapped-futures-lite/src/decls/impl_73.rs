macro_rules! impl_73 {
    () => {
        impl < T , F , Fut > fmt :: Debug for Unfold < T , F , Fut > where T : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Unfold") . field ("state" , & self . state) . field ("fut" , & self . fut) . finish () } }
    };
}

impl_73!();