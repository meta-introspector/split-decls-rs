macro_rules! deps {
    () => {
        IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0!();
    };
}

macro_rules! other_126 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 { pub UnwindData : u32 , pub Anonymous : IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 , }
    };
}

other_126!();