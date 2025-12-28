macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! impl_1135 {
    () => {
        deps!();
        impl < T : fmt :: Debug , P > fmt :: Debug for ChunkBy < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ChunkBy") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1135!();