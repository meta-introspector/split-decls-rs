macro_rules! deps {
    () => {
        Result!();
        Error!();
        Arbitrary!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: EmptyChoose => write ! (f , "`arbitrary::Unstructured::choose` must be given a non-empty set of choices") , Error :: NotEnoughData => write ! (f , "There is not enough underlying raw data to construct an `Arbitrary` instance") , Error :: IncorrectFormat => write ! (f , "The raw data is not of the correct format to construct this type") , } } }
    };
}

impl_1!();