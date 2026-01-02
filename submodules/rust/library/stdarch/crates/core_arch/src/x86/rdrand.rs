mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.x86.rdrand.16"] fn x86_rdrand16_step () -> (u16 , i32) ; # [link_name = "llvm.x86.rdrand.32"] fn x86_rdrand32_step () -> (u32 , i32) ; # [link_name = "llvm.x86.rdseed.16"] fn x86_rdseed16_step () -> (u16 , i32) ; # [link_name = "llvm.x86.rdseed.32"] fn x86_rdseed32_step () -> (u32 , i32) ; }}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _rdrand16_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdrand16_step in module {}", module_path!());
    };
}

mkfn!{
    _rdrand16_step_introspect!();
    # [doc = " Read a hardware generated 16-bit random value and store the result in val."] # [doc = " Returns 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand16_step)"] # [inline] # [target_feature (enable = "rdrand")] # [cfg_attr (test , assert_instr (rdrand))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdrand16_step (val : & mut u16) -> i32 { let (v , flag) = x86_rdrand16_step () ; * val = v ; flag }
}

macro_rules! _rdrand32_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdrand32_step in module {}", module_path!());
    };
}

mkfn!{
    _rdrand32_step_introspect!();
    # [doc = " Read a hardware generated 32-bit random value and store the result in val."] # [doc = " Returns 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand32_step)"] # [inline] # [target_feature (enable = "rdrand")] # [cfg_attr (test , assert_instr (rdrand))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdrand32_step (val : & mut u32) -> i32 { let (v , flag) = x86_rdrand32_step () ; * val = v ; flag }
}

macro_rules! _rdseed16_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdseed16_step in module {}", module_path!());
    };
}

mkfn!{
    _rdseed16_step_introspect!();
    # [doc = " Read a 16-bit NIST SP800-90B and SP800-90C compliant random value and store"] # [doc = " in val. Return 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdseed16_step)"] # [inline] # [target_feature (enable = "rdseed")] # [cfg_attr (test , assert_instr (rdseed))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdseed16_step (val : & mut u16) -> i32 { let (v , flag) = x86_rdseed16_step () ; * val = v ; flag }
}

macro_rules! _rdseed32_step_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _rdseed32_step in module {}", module_path!());
    };
}

mkfn!{
    _rdseed32_step_introspect!();
    # [doc = " Read a 32-bit NIST SP800-90B and SP800-90C compliant random value and store"] # [doc = " in val. Return 1 if a random value was generated, and 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdseed32_step)"] # [inline] # [target_feature (enable = "rdseed")] # [cfg_attr (test , assert_instr (rdseed))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _rdseed32_step (val : & mut u32) -> i32 { let (v , flag) = x86_rdseed32_step () ; * val = v ; flag }
}