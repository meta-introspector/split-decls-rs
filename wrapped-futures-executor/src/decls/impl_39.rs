macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl fmt :: Debug for Task { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Task") . field ("contents" , & "...") . finish () } }
    };
}

impl_39!()