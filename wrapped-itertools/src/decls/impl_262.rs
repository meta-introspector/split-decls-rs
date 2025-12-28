macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl < K , I , F > Debug for ChunkBy < K , I , F > where K : Debug , I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (ChunkBy , inner , index) ; }
    };
}

impl_262!();