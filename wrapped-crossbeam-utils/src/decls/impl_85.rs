macro_rules! deps {
    () => {
        Backoff!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl fmt :: Debug for Backoff { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Backoff") . field ("step" , & self . step) . field ("is_completed" , & self . is_completed ()) . finish () } }
    };
}

impl_85!();