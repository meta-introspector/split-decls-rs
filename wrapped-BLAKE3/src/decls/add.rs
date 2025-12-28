macro_rules! add {
    () => {
        # [inline (always)] fn add (a : v128 , b : v128) -> v128 { i32x4_add (a , b) }
    };
}

add!();