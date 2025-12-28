macro_rules! deps {
    () => {
        BuildError!();
        SmallIndex!();
        ErrorKind!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind { ErrorKind :: StateIDOverflow { max , requested_max } => { write ! (f , "state identifier overflow: failed to create state ID \
                     from {}, which exceeds the max of {}" , requested_max , max ,) } ErrorKind :: PatternIDOverflow { max , requested_max } => { write ! (f , "pattern identifier overflow: failed to create pattern ID \
                     from {}, which exceeds the max of {}" , requested_max , max ,) } ErrorKind :: PatternTooLong { pattern , len } => { write ! (f , "pattern {} with length {} exceeds \
                     the maximum pattern length of {}" , pattern . as_usize () , len , SmallIndex :: MAX . as_usize () ,) } } } }
    };
}

impl_342!()