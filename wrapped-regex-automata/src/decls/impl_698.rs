macro_rules! deps {
    () => {
        UnicodeWordBoundaryError!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl core :: fmt :: Display for UnicodeWordBoundaryError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Unicode-aware \\b and \\B are unavailable because the \
             requisite data tables are missing, please enable the \
             unicode-word-boundary feature") } }
    };
}

impl_698!();