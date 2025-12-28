macro_rules! cpu_subtype_intel {
    () => {
        # [inline] pub const fn cpu_subtype_intel (f : u32 , m : u32) -> u32 { f + (m << 4) }
    };
}

cpu_subtype_intel!();