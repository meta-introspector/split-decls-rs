macro_rules! set4 {
    () => {
        # [inline (always)] fn set4 (a : u32 , b : u32 , c : u32 , d : u32) -> v128 { i32x4 (a as i32 , b as i32 , c as i32 , d as i32) }
    };
}

set4!()