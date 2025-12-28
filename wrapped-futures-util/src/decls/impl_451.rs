macro_rules! impl_451 {
    () => {
        impl < St , Fut > fmt :: Debug for TakeUntil < St , Fut > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TakeUntil") . field ("stream" , & self . stream) . field ("fut" , & self . fut) . finish () } }
    };
}

impl_451!();