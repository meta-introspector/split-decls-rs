macro_rules! impl_501 {
    () => {
        impl < St > fmt :: Debug for BufferUnordered < St > where St : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufferUnordered") . field ("stream" , & self . stream) . field ("in_progress_queue" , & self . in_progress_queue) . field ("max" , & self . max) . finish () } }
    };
}

impl_501!()