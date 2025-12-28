macro_rules! deps {
    () => {
        FromOsStringError!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl fmt :: Display for FromOsStringError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "OsString contains invalid UTF-8: {}" , PathBuf :: from (& self . os_string) . display ()) } }
    };
}

impl_96!()