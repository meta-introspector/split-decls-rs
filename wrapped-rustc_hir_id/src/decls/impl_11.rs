macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Debug for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "HirId({:?}.{:?})" , self . owner , self . local_id) } }
    };
}

impl_11!();