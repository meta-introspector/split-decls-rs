macro_rules! deps {
    () => {
        ThreadId!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ThreadId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ThreadId({})" , self . id . public_id ()) } }
    };
}

impl_327!()