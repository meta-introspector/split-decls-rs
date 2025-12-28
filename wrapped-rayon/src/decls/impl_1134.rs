macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! impl_1134 {
    () => {
        deps!();
        impl < T , P : Clone > Clone for ChunkBy < '_ , T , P > { fn clone (& self) -> Self { ChunkBy { pred : self . pred . clone () , slice : self . slice , } } }
    };
}

impl_1134!();