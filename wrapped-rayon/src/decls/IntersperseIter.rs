macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! IntersperseIter {
    () => {
        deps!();
        struct IntersperseIter < I > where I : Iterator , { base : Fuse < I > , item : I :: Item , clone_first : bool , clone_last : bool , }
    };
}

IntersperseIter!();