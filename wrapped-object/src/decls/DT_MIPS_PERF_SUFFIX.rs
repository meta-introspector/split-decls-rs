macro_rules! DT_MIPS_PERF_SUFFIX {
    () => {
        # [doc = " Default suffix of dso to be added by rld on dlopen() calls."] pub const DT_MIPS_PERF_SUFFIX : u32 = 0x7000_002e ;
    };
}

DT_MIPS_PERF_SUFFIX!()