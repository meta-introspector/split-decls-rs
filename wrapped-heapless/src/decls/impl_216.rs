macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < LenT : LenType > DoubleEndedIterator for Drain < '_ , LenT > { # [inline] fn next_back (& mut self) -> Option < char > { self . iter . next_back () } }
    };
}

impl_216!();