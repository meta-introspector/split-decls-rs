macro_rules! rol {
    () => {
        const fn rol (value : u32 , bits : u32) -> u32 { value . rotate_left (bits) }
    };
}

rol!();