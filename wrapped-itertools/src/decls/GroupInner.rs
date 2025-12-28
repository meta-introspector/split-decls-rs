macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! GroupInner {
    () => {
        deps!();
        # [derive (Clone)] struct GroupInner < K , I , F > where I : Iterator , { key : F , iter : I , current_key : Option < K > , current_elt : Option < I :: Item > , # [doc = " flag set if iterator is exhausted"] done : bool , # [doc = " Index of group we are currently buffering or visiting"] top_group : usize , # [doc = " Least index for which we still have elements buffered"] oldest_buffered_group : usize , # [doc = " Group index for `buffer[0]` -- the slots"] # [doc = " `bottom_group..oldest_buffered_group` are unused and will be erased when"] # [doc = " that range is large enough."] bottom_group : usize , # [doc = " Buffered groups, from `bottom_group` (index 0) to `top_group`."] buffer : Vec < vec :: IntoIter < I :: Item > > , # [doc = " index of last group iter that was dropped,"] # [doc = " `usize::MAX` initially when no group was dropped"] dropped_group : usize , }
    };
}

GroupInner!()