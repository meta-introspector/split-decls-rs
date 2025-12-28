macro_rules! deps {
    () => {
        Fold!();
        FoldChunksWith!();
    };
}

macro_rules! impl_575 {
    () => {
        deps!();
        impl < I : Debug , U : Debug , F > Debug for FoldChunksWith < I , U , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . field ("chunk_size" , & self . chunk_size) . field ("item" , & self . item) . finish () } }
    };
}

impl_575!();