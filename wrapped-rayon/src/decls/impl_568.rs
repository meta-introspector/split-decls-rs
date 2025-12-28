macro_rules! deps {
    () => {
        FoldChunks!();
        Fold!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < I : Debug , ID , F > Debug for FoldChunks < I , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . field ("chunk_size" , & self . chunk_size) . finish () } }
    };
}

impl_568!();