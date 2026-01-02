mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _lzcnt_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _lzcnt_u64 in module {}", module_path!());
    };
}

mkfn!{
    _lzcnt_u64_introspect!();
    # [doc = " Counts the leading most significant zero bits."] # [doc = ""] # [doc = " When the operand is zero, it returns its size in bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_lzcnt_u64)"] # [inline] # [target_feature (enable = "lzcnt")] # [cfg_attr (test , assert_instr (lzcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _lzcnt_u64 (x : u64) -> u64 { x . leading_zeros () as u64 }
}

macro_rules! _popcnt64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _popcnt64 in module {}", module_path!());
    };
}

mkfn!{
    _popcnt64_introspect!();
    # [doc = " Counts the bits that are set."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_popcnt64)"] # [inline] # [target_feature (enable = "popcnt")] # [cfg_attr (test , assert_instr (popcnt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _popcnt64 (x : i64) -> i32 { x . count_ones () as i32 }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: arch :: x86_64 :: * ;}

macro_rules! test_lzcnt_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_lzcnt_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_lzcnt_u64_introspect!();
    # [simd_test (enable = "lzcnt")] unsafe fn test_lzcnt_u64 () { assert_eq ! (_lzcnt_u64 (0b0101_1010) , 57) ; }
}

macro_rules! test_popcnt64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_popcnt64 in module {}", module_path!());
    };
}

mkfn!{
    test_popcnt64_introspect!();
    # [simd_test (enable = "popcnt")] unsafe fn test_popcnt64 () { assert_eq ! (_popcnt64 (0b0101_1010) , 4) ; }
} 
            }}