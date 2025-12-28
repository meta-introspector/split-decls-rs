macro_rules! deps {
    () => {
        NFA!();
        ByteSet!();
        DFA!();
    };
}

macro_rules! ByteClassSet {
    () => {
        deps!();
        # [doc = " A partitioning of bytes into equivalence classes."] # [doc = ""] # [doc = " A byte class set keeps track of an *approximation* of equivalence classes"] # [doc = " of bytes during NFA construction. That is, every byte in an equivalence"] # [doc = " class cannot discriminate between a match and a non-match."] # [doc = ""] # [doc = " For example, in the regex `[ab]+`, the bytes `a` and `b` would be in the"] # [doc = " same equivalence class because it never matters whether an `a` or a `b` is"] # [doc = " seen, and no combination of `a`s and `b`s in the text can discriminate a"] # [doc = " match."] # [doc = ""] # [doc = " Note though that this does not compute the minimal set of equivalence"] # [doc = " classes. For example, in the regex `[ac]+`, both `a` and `c` are in the"] # [doc = " same equivalence class for the same reason that `a` and `b` are in the"] # [doc = " same equivalence class in the aforementioned regex. However, in this"] # [doc = " implementation, `a` and `c` are put into distinct equivalence classes. The"] # [doc = " reason for this is implementation complexity. In the future, we should"] # [doc = " endeavor to compute the minimal equivalence classes since they can have a"] # [doc = " rather large impact on the size of the DFA. (Doing this will likely require"] # [doc = " rethinking how equivalence classes are computed, including changing the"] # [doc = " representation here, which is only able to group contiguous bytes into the"] # [doc = " same equivalence class.)"] # [cfg (feature = "alloc")] # [derive (Clone , Debug)] pub (crate) struct ByteClassSet (ByteSet) ;
    };
}

ByteClassSet!();