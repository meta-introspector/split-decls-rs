mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.x86.rdrand.64"] fn x86_rdrand64_step () -> (u64 , i32) ; # [link_name = "llvm.x86.rdseed.64"] fn x86_rdseed64_step () -> (u64 , i32) ; }}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _rdrand64_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdrand64_step in module {}", module_path!());
    };
}

mkfn!{
    _rdrand64_step_introspect!();
    # [doc = " Read a hardware generated 64-bit random value and store the result in val."] # [doc = " Returns 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand64_step)"] # [inline] # [target_feature (enable = "rdrand")] # [cfg_attr (test , assert_instr (rdrand))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdrand64_step (val : & mut u64) -> i32 { let (v , flag) = x86_rdrand64_step () ; * val = v ; flag }
}

macro_rules! _rdseed64_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdseed64_step in module {}", module_path!());
    };
}

mkfn!{
    _rdseed64_step_introspect!();
    # [doc = " Read a 64-bit NIST SP800-90B and SP800-90C compliant random value and store"] # [doc = " in val. Return 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdseed64_step)"] # [inline] # [target_feature (enable = "rdseed")] # [cfg_attr (test , assert_instr (rdseed))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdseed64_step (val : & mut u64) -> i32 { let (v , flag) = x86_rdseed64_step () ; * val = v ; flag }
}