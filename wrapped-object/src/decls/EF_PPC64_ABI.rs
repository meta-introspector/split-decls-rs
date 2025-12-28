macro_rules! EF_PPC64_ABI {
    () => {
        # [doc = " PowerPC64 bits specifying ABI."] # [doc = ""] # [doc = " 1 for original function descriptor using ABI,"] # [doc = " 2 for revised ABI without function descriptors,"] # [doc = " 0 for unspecified or not using any features affected by the differences."] pub const EF_PPC64_ABI : u32 = 3 ;
    };
}

EF_PPC64_ABI!();