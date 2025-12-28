macro_rules! impl_618 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryFilter < St , Fut , F > where St : TryStream + fmt :: Debug , St :: Ok : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFilter") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . finish () } }
    };
}

impl_618!()