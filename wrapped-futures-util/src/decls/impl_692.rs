macro_rules! impl_692 {
    () => {
        impl < T , F , Fut > fmt :: Debug for TryUnfold < T , F , Fut > where T : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryUnfold") . field ("state" , & self . state) . field ("fut" , & self . fut) . finish () } }
    };
}

impl_692!();