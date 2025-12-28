macro_rules! impl_295 {
    () => {
        impl fmt :: Debug for llvm :: Metadata { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (self as * const Self) . fmt (f) } }
    };
}

impl_295!()