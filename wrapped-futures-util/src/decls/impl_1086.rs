macro_rules! impl_1086 {
    () => {
        impl < T , U > fmt :: Debug for Chain < T , U > where T : fmt :: Debug , U : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Chain") . field ("t" , & self . first) . field ("u" , & self . second) . field ("done_first" , & self . done_first) . finish () } }
    };
}

impl_1086!()