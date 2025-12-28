macro_rules! impl_444 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TakeWhile < St , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TakeWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_taking" , & self . done_taking) . finish () } }
    };
}

impl_444!();