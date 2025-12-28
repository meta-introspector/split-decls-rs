macro_rules! deps {
    () => {
        DecodeSliceError!();
        DecodeError!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl fmt :: Display for DecodeSliceError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: DecodeError (e) => write ! (f , "DecodeError: {}" , e) , Self :: OutputSliceTooSmall => write ! (f , "Output slice too small") , } } }
    };
}

impl_234!()