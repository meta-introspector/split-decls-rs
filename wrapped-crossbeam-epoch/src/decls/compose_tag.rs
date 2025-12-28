macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! compose_tag {
    () => {
        deps!();
        # [doc = " Given a tagged pointer `data`, returns the same pointer, but tagged with `tag`."] # [doc = ""] # [doc = " `tag` is truncated to fit into the unused bits of the pointer to `T`."] # [inline] fn compose_tag < T : ? Sized + Pointable > (ptr : * mut () , tag : usize) -> * mut () { map_addr (ptr , | a | (a & ! low_bits :: < T > ()) | (tag & low_bits :: < T > ())) }
    };
}

compose_tag!()