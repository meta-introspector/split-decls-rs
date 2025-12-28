macro_rules! deps {
    () => {
        HashTable!();
        RawExtractIf!();
    };
}

macro_rules! ExtractIf {
    () => {
        deps!();
        # [doc = " A draining iterator over entries of a `HashTable` which don't satisfy the predicate `f`."] # [doc = ""] # [doc = " This `struct` is created by [`HashTable::extract_if`]. See its"] # [doc = " documentation for more."] # [must_use = "Iterators are lazy unless consumed"] pub struct ExtractIf < 'a , T , F , A : Allocator = Global > { f : F , inner : RawExtractIf < 'a , T , A > , }
    };
}

ExtractIf!()