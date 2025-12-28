macro_rules! g1 {
    () => {
        # [inline (always)] fn g1 (row0 : & mut v128 , row1 : & mut v128 , row2 : & mut v128 , row3 : & mut v128 , m : v128) { * row0 = add (add (* row0 , m) , * row1) ; * row3 = xor (* row3 , * row0) ; * row3 = rot16 (* row3) ; * row2 = add (* row2 , * row3) ; * row1 = xor (* row1 , * row2) ; * row1 = rot12 (* row1) ; }
    };
}

g1!();