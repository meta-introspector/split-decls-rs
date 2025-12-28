macro_rules! deps {
    () => {
        State!();
        TagRef!();
    };
}

macro_rules! TagRefIter {
    () => {
        deps!();
        # [doc = " Like [`TagRef`], but as `Iterator` to support entirely allocation free parsing."] # [doc = " It's particularly useful to dereference only the target chain."] # [derive (Copy , Clone)] pub struct TagRefIter < 'a > { data : & 'a [u8] , state : tag :: ref_iter :: State , }
    };
}

TagRefIter!()