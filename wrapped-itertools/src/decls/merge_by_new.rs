macro_rules! deps {
    () => {
        MergeBy!();
    };
}

macro_rules! merge_by_new {
    () => {
        deps!();
        # [doc = " Create a `MergeBy` iterator."] pub fn merge_by_new < I , J , F > (a : I , b : J , cmp : F) -> MergeBy < I :: IntoIter , J :: IntoIter , F > where I : IntoIterator , J : IntoIterator < Item = I :: Item > , { MergeBy { left : put_back (a . into_iter () . fuse ()) , right : put_back (b . into_iter () . fuse ()) , cmp_fn : cmp , } }
    };
}

merge_by_new!();