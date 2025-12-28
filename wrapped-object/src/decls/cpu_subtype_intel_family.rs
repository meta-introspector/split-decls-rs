macro_rules! cpu_subtype_intel_family {
    () => {
        # [inline] pub const fn cpu_subtype_intel_family (x : u32) -> u32 { x & 15 }
    };
}

cpu_subtype_intel_family!()