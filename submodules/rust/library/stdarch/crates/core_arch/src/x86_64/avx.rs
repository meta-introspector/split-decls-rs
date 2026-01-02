mkuse!{use crate :: { core_arch :: x86 :: * , mem :: transmute } ;}

macro_rules! _mm256_insert_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_insert_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_insert_epi64_introspect!();
    # [doc = " Copies `a` to result, and insert the 64-bit integer `i` into result"] # [doc = " at the location specified by `index`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_insert_epi64)"] # [inline] # [rustc_legacy_const_generics (2)] # [target_feature (enable = "avx")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_insert_epi64 < const INDEX : i32 > (a : __m256i , i : i64) -> __m256i { static_assert_uimm_bits ! (INDEX , 2) ; unsafe { transmute (simd_insert ! (a . as_i64x4 () , INDEX as u32 , i)) } }
}

macro_rules! _mm256_extract_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_extract_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_extract_epi64_introspect!();
    # [doc = " Extracts a 64-bit integer from `a`, selected with `INDEX`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_extract_epi64)"] # [inline] # [target_feature (enable = "avx")] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_extract_epi64 < const INDEX : i32 > (a : __m256i) -> i64 { static_assert_uimm_bits ! (INDEX , 2) ; unsafe { simd_extract ! (a . as_i64x4 () , INDEX as u32) } }
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

macro_rules! test_mm256_insert_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_insert_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_insert_epi64_introspect!();
    # [simd_test (enable = "avx")] unsafe fn test_mm256_insert_epi64 () { let a = _mm256_setr_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_insert_epi64 :: < 3 > (a , 0) ; let e = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_extract_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_extract_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_extract_epi64_introspect!();
    # [simd_test (enable = "avx")] unsafe fn test_mm256_extract_epi64 () { let a = _mm256_setr_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_extract_epi64 :: < 3 > (a) ; assert_eq ! (r , 3) ; }
} 
            }}