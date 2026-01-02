mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mulx_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mulx_u32 in module {}", module_path!());
    };
}

mkfn!{
    _mulx_u32_introspect!();
    # [doc = " Unsigned multiply without affecting flags."] # [doc = ""] # [doc = " Unsigned multiplication of `a` with `b` returning a pair `(lo, hi)` with"] # [doc = " the low half and the high half of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mulx_u32)"] # [inline] # [cfg_attr (all (test , target_arch = "x86_64") , assert_instr (imul))] # [cfg_attr (all (test , target_arch = "x86") , assert_instr (mul))] # [target_feature (enable = "bmi2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mulx_u32 (a : u32 , b : u32 , hi : & mut u32) -> u32 { let result : u64 = (a as u64) * (b as u64) ; * hi = (result >> 32) as u32 ; result as u32 }
}

macro_rules! _bzhi_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bzhi_u32 in module {}", module_path!());
    };
}

mkfn!{
    _bzhi_u32_introspect!();
    # [doc = " Zeroes higher bits of `a` >= `index`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bzhi_u32)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (bzhi))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bzhi_u32 (a : u32 , index : u32) -> u32 { unsafe { x86_bmi2_bzhi_32 (a , index) } }
}

macro_rules! _pdep_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _pdep_u32 in module {}", module_path!());
    };
}

mkfn!{
    _pdep_u32_introspect!();
    # [doc = " Scatter contiguous low order bits of `a` to the result at the positions"] # [doc = " specified by the `mask`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pdep_u32)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (pdep))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _pdep_u32 (a : u32 , mask : u32) -> u32 { unsafe { x86_bmi2_pdep_32 (a , mask) } }
}

macro_rules! _pext_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _pext_u32 in module {}", module_path!());
    };
}

mkfn!{
    _pext_u32_introspect!();
    # [doc = " Gathers the bits of `x` specified by the `mask` into the contiguous low"] # [doc = " order bit positions of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pext_u32)"] # [inline] # [target_feature (enable = "bmi2")] # [cfg_attr (test , assert_instr (pext))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _pext_u32 (a : u32 , mask : u32) -> u32 { unsafe { x86_bmi2_pext_32 (a , mask) } }
}
mkitem!{unsafe extern "C" { # [link_name = "llvm.x86.bmi.bzhi.32"] fn x86_bmi2_bzhi_32 (x : u32 , y : u32) -> u32 ; # [link_name = "llvm.x86.bmi.pdep.32"] fn x86_bmi2_pdep_32 (x : u32 , y : u32) -> u32 ; # [link_name = "llvm.x86.bmi.pext.32"] fn x86_bmi2_pext_32 (x : u32 , y : u32) -> u32 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_pext_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_pext_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_pext_u32_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_pext_u32 () { let n = 0b1011_1110_1001_0011u32 ; let m0 = 0b0110_0011_1000_0101u32 ; let s0 = 0b0000_0000_0011_0101u32 ; let m1 = 0b1110_1011_1110_1111u32 ; let s1 = 0b0001_0111_0100_0011u32 ; assert_eq ! (_pext_u32 (n , m0) , s0) ; assert_eq ! (_pext_u32 (n , m1) , s1) ; }
}

macro_rules! test_pdep_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_pdep_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_pdep_u32_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_pdep_u32 () { let n = 0b1011_1110_1001_0011u32 ; let m0 = 0b0110_0011_1000_0101u32 ; let s0 = 0b0000_0010_0000_0101u32 ; let m1 = 0b1110_1011_1110_1111u32 ; let s1 = 0b1110_1001_0010_0011u32 ; assert_eq ! (_pdep_u32 (n , m0) , s0) ; assert_eq ! (_pdep_u32 (n , m1) , s1) ; }
}

macro_rules! test_bzhi_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bzhi_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_bzhi_u32_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_bzhi_u32 () { let n = 0b1111_0010u32 ; let s = 0b0001_0010u32 ; assert_eq ! (_bzhi_u32 (n , 5) , s) ; }
}

macro_rules! test_mulx_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mulx_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_mulx_u32_introspect!();
    # [simd_test (enable = "bmi2")] unsafe fn test_mulx_u32 () { let a : u32 = 4_294_967_200 ; let b : u32 = 2 ; let mut hi = 0 ; let lo = _mulx_u32 (a , b , & mut hi) ; assert_eq ! (lo , 0b1111_1111_1111_1111_1111_1111_0100_0000u32) ; assert_eq ! (hi , 0b0001u32) ; }
} 
            }}