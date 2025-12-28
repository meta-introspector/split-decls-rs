macro_rules! sh_left {
    () => {
        # [inline (always)] fn sh_left (i : i32) -> i32 { ((i as u32) << 1) as i32 }
    };
}

sh_left!();