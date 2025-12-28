macro_rules! impl_696 {
    () => {
        impl < St , Fut , F > fmt :: Debug for TrySkipWhile < St , Fut , F > where St : TryStream + fmt :: Debug , St :: Ok : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TrySkipWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_skipping" , & self . done_skipping) . finish () } }
    };
}

impl_696!()