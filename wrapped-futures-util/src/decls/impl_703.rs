macro_rules! impl_703 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TryTakeWhile < St , Fut , F > where St : TryStream + fmt :: Debug , St :: Ok : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryTakeWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_taking" , & self . done_taking) . finish () } }
    };
}

impl_703!();