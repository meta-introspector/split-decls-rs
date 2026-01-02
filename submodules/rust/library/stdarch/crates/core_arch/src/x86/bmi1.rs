mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _bextr_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bextr_u32 in module {}", module_path!());
    };
}

mkfn!{
    _bextr_u32_introspect!();
    # [doc = " Extracts bits in range [`start`, `start` + `length`) from `a` into"] # [doc = " the least significant bits of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bextr_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (bextr))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bextr_u32 (a : u32 , start : u32 , len : u32) -> u32 { _bextr2_u32 (a , (start & 0xff_u32) | ((len & 0xff_u32) << 8_u32)) }
}

macro_rules! _bextr2_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bextr2_u32 in module {}", module_path!());
    };
}

mkfn!{
    _bextr2_u32_introspect!();
    # [doc = " Extracts bits of `a` specified by `control` into"] # [doc = " the least significant bits of the result."] # [doc = ""] # [doc = " Bits `[7,0]` of `control` specify the index to the first bit in the range"] # [doc = " to be extracted, and bits `[15,8]` specify the length of the range."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bextr2_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (bextr))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _bextr2_u32 (a : u32 , control : u32) -> u32 { unsafe { x86_bmi_bextr_32 (a , control) } }
}

macro_rules! _andn_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _andn_u32 in module {}", module_path!());
    };
}

mkfn!{
    _andn_u32_introspect!();
    # [doc = " Bitwise logical `AND` of inverted `a` with `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_andn_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (andn))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _andn_u32 (a : u32 , b : u32) -> u32 { ! a & b }
}

macro_rules! _blsi_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsi_u32 in module {}", module_path!());
    };
}

mkfn!{
    _blsi_u32_introspect!();
    # [doc = " Extracts lowest set isolated bit."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsi_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsi))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsi_u32 (x : u32) -> u32 { x & x . wrapping_neg () }
}

macro_rules! _blsmsk_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsmsk_u32 in module {}", module_path!());
    };
}

mkfn!{
    _blsmsk_u32_introspect!();
    # [doc = " Gets mask up to lowest set bit."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsmsk_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsmsk))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsmsk_u32 (x : u32) -> u32 { x ^ (x . wrapping_sub (1_u32)) }
}

macro_rules! _blsr_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsr_u32 in module {}", module_path!());
    };
}

mkfn!{
    _blsr_u32_introspect!();
    # [doc = " Resets the lowest set bit of `x`."] # [doc = ""] # [doc = " If `x` is sets CF."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_blsr_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (blsr))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _blsr_u32 (x : u32) -> u32 { x & (x . wrapping_sub (1)) }
}

macro_rules! _tzcnt_u16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tzcnt_u16 in module {}", module_path!());
    };
}

mkfn!{
    _tzcnt_u16_introspect!();
    # [doc = " Counts the number of trailing least significant zero bits."] # [doc = ""] # [doc = " When the source operand is `0`, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tzcnt_u16)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (tzcnt))] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub fn _tzcnt_u16 (x : u16) -> u16 { x . trailing_zeros () as u16 }
}

macro_rules! _tzcnt_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tzcnt_u32 in module {}", module_path!());
    };
}

mkfn!{
    _tzcnt_u32_introspect!();
    # [doc = " Counts the number of trailing least significant zero bits."] # [doc = ""] # [doc = " When the source operand is `0`, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tzcnt_u32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (tzcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _tzcnt_u32 (x : u32) -> u32 { x . trailing_zeros () }
}

macro_rules! _mm_tzcnt_32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_tzcnt_32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_tzcnt_32_introspect!();
    # [doc = " Counts the number of trailing least significant zero bits."] # [doc = ""] # [doc = " When the source operand is `0`, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_tzcnt_32)"] # [inline] # [target_feature (enable = "bmi1")] # [cfg_attr (test , assert_instr (tzcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_tzcnt_32 (x : u32) -> i32 { x . trailing_zeros () as i32 }
}
mkitem!{unsafe extern "C" { # [link_name = "llvm.x86.bmi.bextr.32"] fn x86_bmi_bextr_32 (x : u32 , y : u32) -> u32 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_bextr_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bextr_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_bextr_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_bextr_u32 () { let r = _bextr_u32 (0b0101_0000u32 , 4 , 4) ; assert_eq ! (r , 0b0000_0101u32) ; }
}

macro_rules! test_andn_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_andn_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_andn_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_andn_u32 () { assert_eq ! (_andn_u32 (0 , 0) , 0) ; assert_eq ! (_andn_u32 (0 , 1) , 1) ; assert_eq ! (_andn_u32 (1 , 0) , 0) ; assert_eq ! (_andn_u32 (1 , 1) , 0) ; let r = _andn_u32 (0b0000_0000u32 , 0b0000_0000u32) ; assert_eq ! (r , 0b0000_0000u32) ; let r = _andn_u32 (0b0000_0000u32 , 0b1111_1111u32) ; assert_eq ! (r , 0b1111_1111u32) ; let r = _andn_u32 (0b1111_1111u32 , 0b0000_0000u32) ; assert_eq ! (r , 0b0000_0000u32) ; let r = _andn_u32 (0b1111_1111u32 , 0b1111_1111u32) ; assert_eq ! (r , 0b0000_0000u32) ; let r = _andn_u32 (0b0100_0000u32 , 0b0101_1101u32) ; assert_eq ! (r , 0b0001_1101u32) ; }
}

macro_rules! test_blsi_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsi_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_blsi_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsi_u32 () { assert_eq ! (_blsi_u32 (0b1101_0000u32) , 0b0001_0000u32) ; }
}

macro_rules! test_blsmsk_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsmsk_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_blsmsk_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsmsk_u32 () { let r = _blsmsk_u32 (0b0011_0000u32) ; assert_eq ! (r , 0b0001_1111u32) ; }
}

macro_rules! test_blsr_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsr_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_blsr_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_blsr_u32 () { let r = _blsr_u32 (0b0011_0000u32) ; assert_eq ! (r , 0b0010_0000u32) ; }
}

macro_rules! test_tzcnt_u16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tzcnt_u16 in module {}", module_path!());
    };
}

mkfn!{
    test_tzcnt_u16_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_tzcnt_u16 () { assert_eq ! (_tzcnt_u16 (0b0000_0001u16) , 0u16) ; assert_eq ! (_tzcnt_u16 (0b0000_0000u16) , 16u16) ; assert_eq ! (_tzcnt_u16 (0b1001_0000u16) , 4u16) ; }
}

macro_rules! test_tzcnt_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tzcnt_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_tzcnt_u32_introspect!();
    # [simd_test (enable = "bmi1")] unsafe fn test_tzcnt_u32 () { assert_eq ! (_tzcnt_u32 (0b0000_0001u32) , 0u32) ; assert_eq ! (_tzcnt_u32 (0b0000_0000u32) , 32u32) ; assert_eq ! (_tzcnt_u32 (0b1001_0000u32) , 4u32) ; }
} 
            }}