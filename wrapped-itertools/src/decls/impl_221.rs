macro_rules! deps {
    () => {
        ExactlyOneError!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < I > Display for ExactlyOneError < I > where I : Iterator , { fn fmt (& self , f : & mut Formatter) -> FmtResult { let additional = self . additional_len () ; if additional > 0 { write ! (f , "got at least 2 elements when exactly one was expected") } else { write ! (f , "got zero elements when exactly one was expected") } } }
    };
}

impl_221!()