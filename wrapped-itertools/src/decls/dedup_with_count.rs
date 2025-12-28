macro_rules! deps {
    () => {
        DedupEq!();
        DedupWithCount!();
    };
}

macro_rules! dedup_with_count {
    () => {
        deps!();
        # [doc = " Create a new `DedupWithCount`."] pub fn dedup_with_count < I > (iter : I) -> DedupWithCount < I > where I : Iterator , { dedup_by_with_count (iter , DedupEq) }
    };
}

dedup_with_count!();