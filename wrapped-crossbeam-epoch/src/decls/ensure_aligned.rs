macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! ensure_aligned {
    () => {
        deps!();
        # [doc = " Panics if the pointer is not properly unaligned."] # [inline] fn ensure_aligned < T : ? Sized + Pointable > (raw : * mut ()) { assert_eq ! (raw as usize & low_bits ::< T > () , 0 , "unaligned pointer") ; }
    };
}

ensure_aligned!();