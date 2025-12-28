macro_rules! deps {
    () => {
        FromPathBufError!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl fmt :: Display for FromPathBufError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "PathBuf contains invalid UTF-8: {}" , self . path . display ()) } }
    };
}

impl_88!()