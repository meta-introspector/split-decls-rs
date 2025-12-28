macro_rules! impl_976 {
    () => {
        impl < Si , Item , U , St , F > fmt :: Debug for WithFlatMap < Si , Item , U , St , F > where Si : fmt :: Debug , St : fmt :: Debug , Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WithFlatMap") . field ("sink" , & self . sink) . field ("stream" , & self . stream) . field ("buffer" , & self . buffer) . finish () } }
    };
}

impl_976!();