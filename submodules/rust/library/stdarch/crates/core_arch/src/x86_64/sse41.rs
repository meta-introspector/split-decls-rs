mkuse!{use crate :: { core_arch :: x86 :: * , mem :: transmute } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_extract_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_extract_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_extract_epi64_introspect!();
    # [doc = " Extracts an 64-bit integer from `a` selected with `IMM1`"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_extract_epi64)"] # [inline] # [target_feature (enable = "sse4.1")] # [cfg_attr (test , assert_instr (pextrq , IMM1 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_extract_epi64 < const IMM1 : i32 > (a : __m128i) -> i64 { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { simd_extract ! (a . as_i64x2 () , IMM1 as u32) } }
}

macro_rules! _mm_insert_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_insert_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_insert_epi64_introspect!();
    # [doc = " Returns a copy of `a` with the 64-bit integer from `i` inserted at a"] # [doc = " location specified by `IMM1`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_insert_epi64)"] # [inline] # [target_feature (enable = "sse4.1")] # [cfg_attr (test , assert_instr (pinsrq , IMM1 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_insert_epi64 < const IMM1 : i32 > (a : __m128i , i : i64) -> __m128i { static_assert_uimm_bits ! (IMM1 , 1) ; unsafe { transmute (simd_insert ! (a . as_i64x2 () , IMM1 as u32 , i)) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: arch :: x86_64 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm_extract_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_extract_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_extract_epi64_introspect!();
    # [simd_test (enable = "sse4.1")] unsafe fn test_mm_extract_epi64 () { let a = _mm_setr_epi64x (0 , 1) ; let r = _mm_extract_epi64 :: < 1 > (a) ; assert_eq ! (r , 1) ; let r = _mm_extract_epi64 :: < 0 > (a) ; assert_eq ! (r , 0) ; }
}

macro_rules! test_mm_insert_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_insert_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_insert_epi64_introspect!();
    # [simd_test (enable = "sse4.1")] unsafe fn test_mm_insert_epi64 () { let a = _mm_set1_epi64x (0) ; let e = _mm_setr_epi64x (0 , 32) ; let r = _mm_insert_epi64 :: < 1 > (a , 32) ; assert_eq_m128i (r , e) ; let e = _mm_setr_epi64x (32 , 0) ; let r = _mm_insert_epi64 :: < 0 > (a , 32) ; assert_eq_m128i (r , e) ; }
} 
            }}