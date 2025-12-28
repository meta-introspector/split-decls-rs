macro_rules! rotate_u128_right {
    () => {
        # [inline (always)] fn rotate_u128_right (x : u128 , i : u32) -> u128 { (x >> i) | (x << (128 - i)) }
    };
}

rotate_u128_right!()