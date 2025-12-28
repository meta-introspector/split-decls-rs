macro_rules! deps {
    () => {
        MultiProduct!();
    };
}

macro_rules! MultiProductIter {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " Holds the state of a single iterator within a `MultiProduct`."] struct MultiProductIter < I > where I : Iterator + Clone , I :: Item : Clone , { iter : I , iter_orig : I , }
    };
}

MultiProductIter!()