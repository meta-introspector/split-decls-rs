macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! decompose_tag {
    () => {
        deps!();
        # [doc = " Decomposes a tagged pointer `data` into the pointer and the tag."] # [inline] fn decompose_tag < T : ? Sized + Pointable > (ptr : * mut ()) -> (* mut () , usize) { (map_addr (ptr , | a | a & ! low_bits :: < T > ()) , ptr as usize & low_bits :: < T > () ,) }
    };
}

decompose_tag!()