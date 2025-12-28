macro_rules! deps {
    () => {
        Dedup!();
        DedupEq!();
    };
}

macro_rules! dedup {
    () => {
        deps!();
        # [doc = " Create a new `Dedup`."] pub fn dedup < I > (iter : I) -> Dedup < I > where I : Iterator , { dedup_by (iter , DedupEq) }
    };
}

dedup!()