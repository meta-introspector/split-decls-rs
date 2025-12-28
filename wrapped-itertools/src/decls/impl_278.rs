macro_rules! deps {
    () => {
        Chunks!();
        Chunk!();
        IntoChunks!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < 'a , I > IntoIterator for & 'a IntoChunks < I > where I : Iterator , I :: Item : 'a , { type Item = Chunk < 'a , I > ; type IntoIter = Chunks < 'a , I > ; fn into_iter (self) -> Self :: IntoIter { Chunks { parent : self } } }
    };
}

impl_278!();