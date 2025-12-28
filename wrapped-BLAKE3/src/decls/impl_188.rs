macro_rules! deps {
    () => {
        ChunkState!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl fmt :: Debug for ChunkState { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("ChunkState") . field ("count" , & self . count ()) . field ("chunk_counter" , & self . chunk_counter) . field ("flags" , & self . flags) . field ("platform" , & self . platform) . finish () } }
    };
}

impl_188!();