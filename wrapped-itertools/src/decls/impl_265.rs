macro_rules! deps {
    () => {
        Groups!();
        ChunkBy!();
        Group!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < 'a , K , I , F > IntoIterator for & 'a ChunkBy < K , I , F > where I : Iterator , I :: Item : 'a , F : FnMut (& I :: Item) -> K , K : PartialEq , { type Item = (K , Group < 'a , K , I , F >) ; type IntoIter = Groups < 'a , K , I , F > ; fn into_iter (self) -> Self :: IntoIter { Groups { parent : self } } }
    };
}

impl_265!()