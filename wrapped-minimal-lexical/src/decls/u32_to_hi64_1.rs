macro_rules! u32_to_hi64_1 {
    () => {
        # [doc = " Shift 32-bit integer to high 64-bits."] # [inline] pub fn u32_to_hi64_1 (r0 : u32) -> (u64 , bool) { u64_to_hi64_1 (r0 as u64) }
    };
}

u32_to_hi64_1!();