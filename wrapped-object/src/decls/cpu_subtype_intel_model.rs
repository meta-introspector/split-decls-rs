macro_rules! cpu_subtype_intel_model {
    () => {
        # [inline] pub const fn cpu_subtype_intel_model (x : u32) -> u32 { x >> 4 }
    };
}

cpu_subtype_intel_model!()