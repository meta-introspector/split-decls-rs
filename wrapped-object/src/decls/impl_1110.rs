macro_rules! deps {
    () => {
        Result!();
        SegmentId!();
    };
}

macro_rules! impl_1110 {
    () => {
        deps!();
        impl fmt :: Debug for SegmentId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "SegmentId({})" , self . 0) } }
    };
}

impl_1110!();