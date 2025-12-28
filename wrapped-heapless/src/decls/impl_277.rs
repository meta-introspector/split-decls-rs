macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < T , LenT : LenType > DoubleEndedIterator for Drain < '_ , T , LenT > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (core :: ptr :: from_ref (elt)) }) } }
    };
}

impl_277!()