macro_rules! impl_958 {
    () => {
        impl < Si , St > fmt :: Debug for SendAll < '_ , Si , St > where Si : fmt :: Debug + ? Sized , St : fmt :: Debug + TryStream , St :: Ok : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SendAll") . field ("sink" , & self . sink) . field ("stream" , & self . stream) . field ("buffered" , & self . buffered) . finish () } }
    };
}

impl_958!()