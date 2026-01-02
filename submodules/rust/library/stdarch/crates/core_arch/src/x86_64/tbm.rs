mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "C" { # [link_name = "llvm.x86.tbm.bextri.u64"] fn bextri_u64 (a : u64 , control : u64) -> u64 ; }}

macro_rules! _bextri_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bextri_u64 in module {}", module_path!());
    };
}

mkfn!{
    _bextri_u64_introspect!();
    # [doc = " Extracts bits of `a` specified by `control` into"] # [doc = " the least significant bits of the result."] # [doc = ""] # [doc = " Bits `[7,0]` of `control` specify the index to the first bit in the range to"] # [doc = " be extracted, and bits `[15,8]` specify the length of the range. For any bit"] # [doc = " position in the specified range that lie beyond the MSB of the source operand,"] # [doc = " zeroes will be written. If the range is empty, the result is zero."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (bextr , CONTROL = 0x0404))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _bextri_u64 < const CONTROL : u64 > (a : u64) -> u64 { static_assert_uimm_bits ! (CONTROL , 16) ; unsafe { bextri_u64 (a , CONTROL) } }
}

macro_rules! _blcfill_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blcfill_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blcfill_u64_introspect!();
    # [doc = " Clears all bits below the least significant zero bit of `x`."] # [doc = ""] # [doc = " If there is no zero bit in `x`, it returns zero."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blcfill))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blcfill_u64 (x : u64) -> u64 { x & x . wrapping_add (1) }
}

macro_rules! _blci_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blci_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blci_u64_introspect!();
    # [doc = " Sets all bits of `x` to 1 except for the least significant zero bit."] # [doc = ""] # [doc = " If there is no zero bit in `x`, it sets all bits."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blci))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blci_u64 (x : u64) -> u64 { x | ! x . wrapping_add (1) }
}

macro_rules! _blcic_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blcic_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blcic_u64_introspect!();
    # [doc = " Sets the least significant zero bit of `x` and clears all other bits."] # [doc = ""] # [doc = " If there is no zero bit in `x`, it returns zero."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blcic))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blcic_u64 (x : u64) -> u64 { ! x & x . wrapping_add (1) }
}

macro_rules! _blcmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blcmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blcmsk_u64_introspect!();
    # [doc = " Sets the least significant zero bit of `x` and clears all bits above"] # [doc = " that bit."] # [doc = ""] # [doc = " If there is no zero bit in `x`, it sets all the bits."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blcmsk))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blcmsk_u64 (x : u64) -> u64 { x ^ x . wrapping_add (1) }
}

macro_rules! _blcs_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blcs_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blcs_u64_introspect!();
    # [doc = " Sets the least significant zero bit of `x`."] # [doc = ""] # [doc = " If there is no zero bit in `x`, it returns `x`."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blcs))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blcs_u64 (x : u64) -> u64 { x | x . wrapping_add (1) }
}

macro_rules! _blsfill_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsfill_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blsfill_u64_introspect!();
    # [doc = " Sets all bits of `x` below the least significant one."] # [doc = ""] # [doc = " If there is no set bit in `x`, it sets all the bits."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blsfill))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blsfill_u64 (x : u64) -> u64 { x | x . wrapping_sub (1) }
}

macro_rules! _blsic_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _blsic_u64 in module {}", module_path!());
    };
}

mkfn!{
    _blsic_u64_introspect!();
    # [doc = " Clears least significant bit and sets all other bits."] # [doc = ""] # [doc = " If there is no set bit in `x`, it sets all the bits."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (blsic))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _blsic_u64 (x : u64) -> u64 { ! x | x . wrapping_sub (1) }
}

macro_rules! _t1mskc_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _t1mskc_u64 in module {}", module_path!());
    };
}

mkfn!{
    _t1mskc_u64_introspect!();
    # [doc = " Clears all bits below the least significant zero of `x` and sets all other"] # [doc = " bits."] # [doc = ""] # [doc = " If the least significant bit of `x` is `0`, it sets all bits."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (t1mskc))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _t1mskc_u64 (x : u64) -> u64 { ! x | x . wrapping_add (1) }
}

macro_rules! _tzmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tzmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    _tzmsk_u64_introspect!();
    # [doc = " Sets all bits below the least significant one of `x` and clears all other"] # [doc = " bits."] # [doc = ""] # [doc = " If the least significant bit of `x` is 1, it returns zero."] # [inline] # [target_feature (enable = "tbm")] # [cfg_attr (test , assert_instr (tzmsk))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _tzmsk_u64 (x : u64) -> u64 { ! x & x . wrapping_sub (1) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86_64 :: * ;}

macro_rules! test_bextri_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bextri_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_bextri_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_bextri_u64 () { assert_eq ! (_bextri_u64 ::< 0x0404 > (0b0101_0000u64) , 0b0000_0101u64) ; }
}

macro_rules! test_blcfill_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blcfill_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blcfill_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blcfill_u64 () { assert_eq ! (_blcfill_u64 (0b0101_0111u64) , 0b0101_0000u64) ; assert_eq ! (_blcfill_u64 (0b1111_1111u64) , 0u64) ; }
}

macro_rules! test_blci_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blci_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blci_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blci_u64 () { assert_eq ! (_blci_u64 (0b0101_0000u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110u64) ; assert_eq ! (_blci_u64 (0b1111_1111u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110_1111_1111u64) ; }
}

macro_rules! test_blcic_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blcic_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blcic_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blcic_u64 () { assert_eq ! (_blcic_u64 (0b0101_0001u64) , 0b0000_0010u64) ; assert_eq ! (_blcic_u64 (0b1111_1111u64) , 0b1_0000_0000u64) ; }
}

macro_rules! test_blcmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blcmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blcmsk_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blcmsk_u64 () { assert_eq ! (_blcmsk_u64 (0b0101_0001u64) , 0b0000_0011u64) ; assert_eq ! (_blcmsk_u64 (0b1111_1111u64) , 0b1_1111_1111u64) ; }
}

macro_rules! test_blcs_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blcs_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blcs_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blcs_u64 () { assert_eq ! (_blcs_u64 (0b0101_0001u64) , 0b0101_0011u64) ; assert_eq ! (_blcs_u64 (0b1111_1111u64) , 0b1_1111_1111u64) ; }
}

macro_rules! test_blsfill_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsfill_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blsfill_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blsfill_u64 () { assert_eq ! (_blsfill_u64 (0b0101_0100u64) , 0b0101_0111u64) ; assert_eq ! (_blsfill_u64 (0u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111u64) ; }
}

macro_rules! test_blsic_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_blsic_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_blsic_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_blsic_u64 () { assert_eq ! (_blsic_u64 (0b0101_0100u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1011u64) ; assert_eq ! (_blsic_u64 (0u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111u64) ; }
}

macro_rules! test_t1mskc_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_t1mskc_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_t1mskc_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_t1mskc_u64 () { assert_eq ! (_t1mskc_u64 (0b0101_0111u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1000u64) ; assert_eq ! (_t1mskc_u64 (0u64) , 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111u64) ; }
}

macro_rules! test_tzmsk_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tzmsk_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_tzmsk_u64_introspect!();
    # [simd_test (enable = "tbm")] unsafe fn test_tzmsk_u64 () { assert_eq ! (_tzmsk_u64 (0b0101_1000u64) , 0b0000_0111u64) ; assert_eq ! (_tzmsk_u64 (0b0101_1001u64) , 0b0000_0000u64) ; }
} 
            }}