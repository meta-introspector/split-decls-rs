macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a > iter :: FusedIterator for LinesWithTerminator < 'a > { }
    };
}

impl_109!();