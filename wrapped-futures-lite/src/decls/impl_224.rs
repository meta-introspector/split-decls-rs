macro_rules! impl_224 {
    () => {
        impl < W : fmt :: Debug > fmt :: Debug for BufWriter < W > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufWriter") . field ("writer" , & self . inner) . field ("buf" , & self . buf) . finish () } }
    };
}

impl_224!();