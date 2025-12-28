macro_rules! impl_967 {
    () => {
        impl < Si , Item , U , Fut , F > fmt :: Debug for With < Si , Item , U , Fut , F > where Si : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("With") . field ("sink" , & self . sink) . field ("state" , & self . state) . finish () } }
    };
}

impl_967!();