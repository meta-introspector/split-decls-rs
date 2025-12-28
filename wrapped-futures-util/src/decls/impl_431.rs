macro_rules! impl_431 {
    () => {
        impl < St , Fut , F > fmt :: Debug for SkipWhile < St , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SkipWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_skipping" , & self . done_skipping) . finish () } }
    };
}

impl_431!()