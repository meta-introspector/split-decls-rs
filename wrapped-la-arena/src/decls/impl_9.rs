macro_rules! deps {
    () => {
        ArenaMapIter!();
        ArenaMap!();
        Idx!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T , V > ArenaMapIter < Idx < T > , V > { fn mapper ((idx , o) : (usize , Option < V >)) -> Option < (Idx < T > , V) > { Some ((ArenaMap :: < Idx < T > , V > :: from_idx (idx) , o ?)) } }
    };
}

impl_9!();