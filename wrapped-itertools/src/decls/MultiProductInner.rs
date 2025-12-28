macro_rules! deps {
    () => {
        MultiProduct!();
        MultiProductIter!();
    };
}

macro_rules! MultiProductInner {
    () => {
        deps!();
        # [derive (Clone)] # [doc = " Internals for `MultiProduct`."] struct MultiProductInner < I > where I : Iterator + Clone , I :: Item : Clone , { # [doc = " Holds the iterators."] iters : Vec < MultiProductIter < I > > , # [doc = " Not populated at the beginning then it holds the current item of each iterator."] cur : CurrentItems < Vec < I :: Item > > , }
    };
}

MultiProductInner!();