macro_rules! impl_406 {
    () => {
        impl < St > fmt :: Debug for Peek < '_ , St > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Peek") . field ("inner" , & self . inner) . finish () } }
    };
}

impl_406!();