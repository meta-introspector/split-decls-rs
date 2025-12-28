macro_rules! deps {
    () => {
        KMerge!();
    };
}

macro_rules! HeadTail {
    () => {
        deps!();
        # [doc = " Head element and Tail iterator pair"] # [doc = ""] # [doc = " `PartialEq`, `Eq`, `PartialOrd` and `Ord` are implemented by comparing sequences based on"] # [doc = " first items (which are guaranteed to exist)."] # [doc = ""] # [doc = " The meanings of `PartialOrd` and `Ord` are reversed so as to turn the heap used in"] # [doc = " `KMerge` into a min-heap."] # [derive (Debug)] struct HeadTail < I > where I : Iterator , { head : I :: Item , tail : I , }
    };
}

HeadTail!()