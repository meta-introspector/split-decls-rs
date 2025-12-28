macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! chunk_index {
    () => {
        deps!();
        # [inline] fn chunk_index < T : Idx > (elem : T) -> usize { elem . index () / CHUNK_BITS }
    };
}

chunk_index!();