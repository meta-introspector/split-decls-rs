macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl fmt :: Debug for List { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . iter () . map (String :: from_utf8_lossy)) . finish () } }
    };
}

impl_105!()