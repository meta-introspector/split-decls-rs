macro_rules! u32_to_hi64_3 {
    () => {
        # [doc = " Shift 3 32-bit integers to high 64-bits."] # [inline] pub fn u32_to_hi64_3 (r0 : u32 , r1 : u32 , r2 : u32) -> (u64 , bool) { let r0 = r0 as u64 ; let r1 = (r1 as u64) << 32 ; let r2 = r2 as u64 ; u64_to_hi64_2 (r0 , r1 | r2) }
    };
}

u32_to_hi64_3!();