mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _bextr_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bextr_u64 in module {}", module_path!());
    };
}

mkfn!{
    _bextr_u64_introspect!();
    # [doc = " Extracts bits in range [`start`, `start` + `length`) from `a` into"] # [doc = " the least significant bits of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bextr_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (bextr))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bextr_u64 (a : u64 , start : u32 , len : u32) -> u64 { _bextr2_u64 (a , ((start & 0xff) | ((len & 0xff) << 8)) as u64) }
}

macro_rules! _bextr2_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bextr2_u64 in module {}", module_path!());
    };
}

mkfn!{
    _bextr2_u64_introspect!();
    # [doc = " Extracts bits of `a` specified by `control` into"] # [doc = " the least significant bits of the result."] # [doc = ""] # [doc = " Bits `[7,0]` of `control` specify the index to the first bit in the range"] # [doc = " to be extracted, and bits `[15,8]` specify the length of the range."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bextr2_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (bextr))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bextr2_u64 (a : u64 , control : u64) -> u64 { unsafe { x86_bmi_bextr_64 (a , control) } }
}

macro_rules! _andn_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _andn_u64 in module {}", module_path!());
    };
}

mkfn!{
    _andn_u64_introspect!();
    # [doc = " Bitwise logical `AND` of inverted `a` with `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_andn_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (andn))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _andn_u64 (a : u64 , b : u64) -> u64 { ! a & b }
}

macro_rules! _blsi_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsi_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blsi_u64_introspect!();
    # [doc = " Extracts lowest set isolated bit."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsi_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsi))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsi_u64 (x : u64) -> u64 { x & x . wrapping_neg () }
}

macro_rules! _blsmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blsmsk_u64_introspect!();
    # [doc = " Gets mask up to lowest set bit."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsmsk_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsmsk))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsmsk_u64 (x : u64) -> u64 { x ^ (x . wrapping_sub (1_u64)) }
}

macro_rules! _blsr_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsr_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blsr_u64_introspect!();
    # [doc = " Resets the lowest set bit of `x`."] # [doc = ""] # [doc = " If `x` is sets CF."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsr_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsr))] # [cfg (not (target_arch = "x86"))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsr_u64 (x : u64) -> u64 { x & (x . wrapping_sub (1)) }
}

macro_rules! _tzcnt_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tzcnt_u64 in module {}", module_path!());
    };
}

mkfn!{
    _tzcnt_u64_introspect!();
    # [doc = " Counts the number of trailing least significant zero bits."] # [doc = ""] # [doc = " When the source operand is `0`, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tzcnt_u64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (tzcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _tzcnt_u64 (x : u64) -> u64 { x . trailing_zeros () as u64 }
}

macro_rules! _mm_tzcnt_64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_tzcnt_64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_tzcnt_64_introspect!();
    # [doc = " Counts the number of trailing least significant zero bits."] # [doc = ""] # [doc = " When the source operand is `0`, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_tzcnt_64)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (tzcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_tzcnt_64 (x : u64) -> i64 { x . trailing_zeros () as i64 }
}
mkitem!{unsafe extern "C" { # [link_name = "llvm.x86.bmi.bextr.64"] fn x86_bmi_bextr_64 (x : u64 , y : u64) -> u64 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: { x86 :: * , x86_64 :: * } ;}

macro_rules! test_bextr_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bextr_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_bextr_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_bextr_u64 () { let r = _bextr_u64 (0b0101_0000u64 , 4 , 4) ; assert_eq ! (r , 0b0000_0101u64) ; }
}

macro_rules! test_andn_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_andn_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_andn_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_andn_u64 () { assert_eq ! (_andn_u64 (0 , 0) , 0) ; assert_eq ! (_andn_u64 (0 , 1) , 1) ; assert_eq ! (_andn_u64 (1 , 0) , 0) ; assert_eq ! (_andn_u64 (1 , 1) , 0) ; let r = _andn_u64 (0b0000_0000u64 , 0b0000_0000u64) ; assert_eq ! (r , 0b0000_0000u64) ; let r = _andn_u64 (0b0000_0000u64 , 0b1111_1111u64) ; assert_eq ! (r , 0b1111_1111u64) ; let r = _andn_u64 (0b1111_1111u64 , 0b0000_0000u64) ; assert_eq ! (r , 0b0000_0000u64) ; let r = _andn_u64 (0b1111_1111u64 , 0b1111_1111u64) ; assert_eq ! (r , 0b0000_0000u64) ; let r = _andn_u64 (0b0100_0000u64 , 0b0101_1101u64) ; assert_eq ! (r , 0b0001_1101u64) ; }
}

macro_rules! test_blsi_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsi_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blsi_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsi_u64 () { assert_eq ! (_blsi_u64 (0b1101_0000u64) , 0b0001_0000u64) ; }
}

macro_rules! test_blsmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blsmsk_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsmsk_u64 () { let r = _blsmsk_u64 (0b0011_0000u64) ; assert_eq ! (r , 0b0001_1111u64) ; }
}

macro_rules! test_blsr_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsr_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blsr_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsr_u64 () { let r = _blsr_u64 (0b0011_0000u64) ; assert_eq ! (r , 0b0010_0000u64) ; }
}

macro_rules! test_tzcnt_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tzcnt_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_tzcnt_u64_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_tzcnt_u64 () { assert_eq ! (_tzcnt_u64 (0b0000_0001u64) , 0u64) ; assert_eq ! (_tzcnt_u64 (0b0000_0000u64) , 64u64) ; assert_eq ! (_tzcnt_u64 (0b1001_0000u64) , 4u64) ; }
} 
            }}