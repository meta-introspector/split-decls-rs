macro_rules! xor {
    () => {
        # [inline (always)] fn xor (a : v128 , b : v128) -> v128 { v128_xor (a , b) }
    };
}

xor!()