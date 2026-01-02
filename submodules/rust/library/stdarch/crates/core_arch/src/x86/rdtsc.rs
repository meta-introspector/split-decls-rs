mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdtsc in module {}", module_path!());
    };
}

mkfn!{
    _rdtsc_introspect!();
    # [doc = " Reads the current value of the processor’s time-stamp counter."] # [doc = ""] # [doc = " The processor monotonically increments the time-stamp counter MSR"] # [doc = " every clock cycle and resets it to 0 whenever the processor is"] # [doc = " reset."] # [doc = ""] # [doc = " The RDTSC instruction is not a serializing instruction. It does"] # [doc = " not necessarily wait until all previous instructions have been"] # [doc = " executed before reading the counter. Similarly, subsequent"] # [doc = " instructions may begin execution before the read operation is"] # [doc = " performed."] # [doc = ""] # [doc = " On processors that support the Intel 64 architecture, the"] # [doc = " high-order 32 bits of each of RAX and RDX are cleared."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdtsc)"] # [inline] # [cfg_attr (test , assert_instr (rdtsc))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdtsc () -> u64 { rdtsc () }
}

macro_rules! __rdtscp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rdtscp in module {}", module_path!());
    };
}

mkfn!{
    __rdtscp_introspect!();
    # [doc = " Reads the current value of the processor’s time-stamp counter and"] # [doc = " the `IA32_TSC_AUX MSR`."] # [doc = ""] # [doc = " The processor monotonically increments the time-stamp counter MSR"] # [doc = " every clock cycle and resets it to 0 whenever the processor is"] # [doc = " reset."] # [doc = ""] # [doc = " The RDTSCP instruction waits until all previous instructions have"] # [doc = " been executed before reading the counter. However, subsequent"] # [doc = " instructions may begin execution before the read operation is"] # [doc = " performed."] # [doc = ""] # [doc = " On processors that support the Intel 64 architecture, the"] # [doc = " high-order 32 bits of each of RAX, RDX, and RCX are cleared."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__rdtscp)"] # [inline] # [cfg_attr (test , assert_instr (rdtscp))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn __rdtscp (aux : * mut u32) -> u64 { let (tsc , auxval) = rdtscp () ; * aux = auxval ; tsc }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.x86.rdtsc"] fn rdtsc () -> u64 ; # [link_name = "llvm.x86.rdtscp"] fn rdtscp () -> (u64 , u32) ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_rdtsc in module {}", module_path!());
    };
}

mkfn!{
    test_rdtsc_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_rdtsc () { let r = _rdtsc () ; assert_ne ! (r , 0) ; }
}

macro_rules! test_rdtscp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_rdtscp in module {}", module_path!());
    };
}

mkfn!{
    test_rdtscp_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_rdtscp () { let mut aux = 0 ; let r = __rdtscp (& mut aux) ; assert_ne ! (r , 0) ; }
} 
            }}