macro_rules! set1 {
    () => {
        # [inline (always)] fn set1 (x : u32) -> v128 { i32x4_splat (x as i32) }
    };
}

set1!()