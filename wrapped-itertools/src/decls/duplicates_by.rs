macro_rules! deps {
    () => {
        DuplicatesBy!();
    };
}

macro_rules! duplicates_by {
    () => {
        deps!();
        # [doc = " Create a new `DuplicatesBy` iterator."] pub fn duplicates_by < I , Key , F > (iter : I , f : F) -> DuplicatesBy < I , Key , F > where Key : Eq + Hash , F : FnMut (& I :: Item) -> Key , I : Iterator , { DuplicatesBy :: new (iter , private :: ByFn (f)) }
    };
}

duplicates_by!();