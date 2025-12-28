macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! ExtractIf {
    () => {
        deps!();
        # [cfg (feature = "extract_if")] # [doc = " An iterator which uses a closure to determine if an element should be removed."] # [doc = ""] # [doc = " Returned from [`SmallVec::extract_if`][1]."] # [doc = ""] # [doc = " [1]: struct.SmallVec.html#method.extract_if"] pub struct ExtractIf < 'a , T , const N : usize , F > where F : FnMut (& mut T) -> bool , { vec : & 'a mut SmallVec < T , N > , # [doc = " The index of the item that will be inspected by the next call to `next`."] idx : usize , # [doc = " Elements at and beyond this point will be retained. Must be equal or smaller than `old_len`."] end : usize , # [doc = " The number of items that have been drained (removed) thus far."] del : usize , # [doc = " The original length of `vec` prior to draining."] old_len : usize , # [doc = " The filter test predicate."] pred : F , }
    };
}

ExtractIf!();