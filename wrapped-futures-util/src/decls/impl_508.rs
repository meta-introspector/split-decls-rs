macro_rules! impl_508 {
    () => {
        impl < St > fmt :: Debug for Buffered < St > where St : Stream + fmt :: Debug , St :: Item : Future , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Buffered") . field ("stream" , & self . stream) . field ("in_progress_queue" , & self . in_progress_queue) . field ("max" , & self . max) . finish () } }
    };
}

impl_508!()