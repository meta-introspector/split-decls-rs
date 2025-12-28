macro_rules! deps {
    () => {
        ChunkByMut!();
    };
}

macro_rules! impl_1139 {
    () => {
        deps!();
        impl < T : fmt :: Debug , P > fmt :: Debug for ChunkByMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ChunkByMut") . field ("slice" , & self . slice) . finish () } }
    };
}

impl_1139!();