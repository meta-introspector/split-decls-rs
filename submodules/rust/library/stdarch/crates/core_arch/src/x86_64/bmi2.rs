mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mulx_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mulx_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mulx_u64_introspect!();
    # [doc = " Unsigned multiply without affecting flags."] # [doc = ""] # [doc = " Unsigned multiplication of `a` with `b` returning a pair `(lo, hi)` with"] # [doc = " the low half and the high half of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mulx_u64)"] # [inline] # [cfg_attr (test , assert_instr (mul))] # [target_feature (enable = "bmi2")] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mulx_u64 (a : u64 , b : u64 , hi : & mut u64) -> u64 { let result : u128 = (a as u128) * (b as u128) ; * hi = (result >> 64) as u64 ; result as u64 }
}

macro_rules! _bzhi_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bzhi_u64 in module {}", module_path!());
    };
}

mkfn!{
    _bzhi_u64_introspect!();
    # [doc = " Zeroes higher bits of `a` >= `index`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bzhi_u64)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (bzhi))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bzhi_u64 (a : u64 , index : u32) -> u64 { unsafe { x86_bmi2_bzhi_64 (a , index as u64) } }
}

macro_rules! _pdep_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _pdep_u64 in module {}", module_path!());
    };
}

mkfn!{
    _pdep_u64_introspect!();
    # [doc = " Scatter contiguous low order bits of `a` to the result at the positions"] # [doc = " specified by the `mask`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pdep_u64)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (pdep))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _pdep_u64 (a : u64 , mask : u64) -> u64 { unsafe { x86_bmi2_pdep_64 (a , mask) } }
}

macro_rules! _pext_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _pext_u64 in module {}", module_path!());
    };
}

mkfn!{
    _pext_u64_introspect!();
    # [doc = " Gathers the bits of `x` specified by the `mask` into the contiguous low"] # [doc = " order bit positions of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pext_u64)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (pext))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _pext_u64 (a : u64 , mask : u64) -> u64 { unsafe { x86_bmi2_pext_64 (a , mask) } }
}
mkitem!{unsafe extern "C" { # [link_name = "llvm.x86.bmi.bzhi.64"] fn x86_bmi2_bzhi_64 (x : u64 , y : u64) -> u64 ; # [link_name = "llvm.x86.bmi.pdep.64"] fn x86_bmi2_pdep_64 (x : u64 , y : u64) -> u64 ; # [link_name = "llvm.x86.bmi.pext.64"] fn x86_bmi2_pext_64 (x : u64 , y : u64) -> u64 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86_64 :: * ;}

macro_rules! test_pext_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_pext_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_pext_u64_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_pext_u64 () { let n = 0b1011_1110_1001_0011u64 ; let m0 = 0b0110_0011_1000_0101u64 ; let s0 = 0b0000_0000_0011_0101u64 ; let m1 = 0b1110_1011_1110_1111u64 ; let s1 = 0b0001_0111_0100_0011u64 ; assert_eq ! (_pext_u64 (n , m0) , s0) ; assert_eq ! (_pext_u64 (n , m1) , s1) ; }
}

macro_rules! test_pdep_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_pdep_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_pdep_u64_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_pdep_u64 () { let n = 0b1011_1110_1001_0011u64 ; let m0 = 0b0110_0011_1000_0101u64 ; let s0 = 0b0000_0010_0000_0101u64 ; let m1 = 0b1110_1011_1110_1111u64 ; let s1 = 0b1110_1001_0010_0011u64 ; assert_eq ! (_pdep_u64 (n , m0) , s0) ; assert_eq ! (_pdep_u64 (n , m1) , s1) ; }
}

macro_rules! test_bzhi_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bzhi_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_bzhi_u64_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_bzhi_u64 () { let n = 0b1111_0010u64 ; let s = 0b0001_0010u64 ; assert_eq ! (_bzhi_u64 (n , 5) , s) ; }
}

macro_rules! test_mulx_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mulx_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mulx_u64_introspect!();
    # [simd_test (enable = "bmi2")] # [rustfmt :: skip] unsafe fn test_mulx_u64 () { let a : u64 = 9_223_372_036_854_775_800 ; let b : u64 = 100 ; let mut hi = 0 ; let lo = _mulx_u64 (a , b , & mut hi) ; assert_eq ! (lo , 0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64) ; assert_eq ! (hi , 0b00110001u64) ; }
} 
            }}