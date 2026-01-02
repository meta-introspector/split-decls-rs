mkuse!{use crate :: core_arch :: simd :: * ;}
mkuse!{use crate :: core_arch :: x86 :: __m128i ;}
mkuse!{use crate :: core_arch :: x86 :: __m256i ;}
mkuse!{use crate :: core_arch :: x86 :: __m512i ;}
mkuse!{use crate :: core_arch :: x86 :: __mmask8 ;}
mkuse!{use crate :: core_arch :: x86 :: __mmask16 ;}
mkuse!{use crate :: intrinsics :: simd :: { simd_ctpop , simd_select_bitmask } ;}
mkuse!{use crate :: mem :: transmute ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm512_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm512_popcnt_epi32 (a : __m512i) -> __m512i { unsafe { transmute (simd_ctpop (a . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm512_maskz_popcnt_epi32 (k : __mmask16 , a : __m512i) -> __m512i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x16 ()) , i32x16 :: ZERO ,)) } }
}

macro_rules! _mm512_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm512_mask_popcnt_epi32 (src : __m512i , k : __mmask16 , a : __m512i) -> __m512i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x16 ()) , src . as_i32x16 () ,)) } }
}

macro_rules! _mm256_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm256_popcnt_epi32 (a : __m256i) -> __m256i { unsafe { transmute (simd_ctpop (a . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm256_maskz_popcnt_epi32 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x8 ()) , i32x8 :: ZERO ,)) } }
}

macro_rules! _mm256_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm256_mask_popcnt_epi32 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x8 ()) , src . as_i32x8 () ,)) } }
}

macro_rules! _mm_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm_popcnt_epi32 (a : __m128i) -> __m128i { unsafe { transmute (simd_ctpop (a . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm_maskz_popcnt_epi32 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x4 ()) , i32x4 :: ZERO ,)) } }
}

macro_rules! _mm_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_popcnt_epi32_introspect!();
    # [doc = " For each packed 32-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_popcnt_epi32)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntd))] pub fn _mm_mask_popcnt_epi32 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i32x4 ()) , src . as_i32x4 () ,)) } }
}

macro_rules! _mm512_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm512_popcnt_epi64 (a : __m512i) -> __m512i { unsafe { transmute (simd_ctpop (a . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm512_maskz_popcnt_epi64 (k : __mmask8 , a : __m512i) -> __m512i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x8 ()) , i64x8 :: ZERO ,)) } }
}

macro_rules! _mm512_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm512_mask_popcnt_epi64 (src : __m512i , k : __mmask8 , a : __m512i) -> __m512i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x8 ()) , src . as_i64x8 () ,)) } }
}

macro_rules! _mm256_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm256_popcnt_epi64 (a : __m256i) -> __m256i { unsafe { transmute (simd_ctpop (a . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm256_maskz_popcnt_epi64 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x4 ()) , i64x4 :: ZERO ,)) } }
}

macro_rules! _mm256_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm256_mask_popcnt_epi64 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x4 ()) , src . as_i64x4 () ,)) } }
}

macro_rules! _mm_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm_popcnt_epi64 (a : __m128i) -> __m128i { unsafe { transmute (simd_ctpop (a . as_i64x2 ())) } }
}

macro_rules! _mm_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are zeroed in the result if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm_maskz_popcnt_epi64 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x2 ()) , i64x2 :: ZERO ,)) } }
}

macro_rules! _mm_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_popcnt_epi64_introspect!();
    # [doc = " For each packed 64-bit integer maps the value to the number of logical 1 bits."] # [doc = ""] # [doc = " Uses the writemask in k - elements are copied from src if the corresponding mask bit is not set."] # [doc = " Otherwise the computation result is written into the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_popcnt_epi64)"] # [inline] # [target_feature (enable = "avx512vpopcntdq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpopcntq))] pub fn _mm_mask_popcnt_epi64 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { transmute (simd_select_bitmask (k , simd_ctpop (a . as_i64x2 ()) , src . as_i64x2 () ,)) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_mm512_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_popcnt_epi32 () { let test_data = _mm512_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100 , 0x40_00_00_00 , 103 , 371 , 552 , 432_948 , 818_826_998 , 255 , 256 ,) ; let actual_result = _mm512_popcnt_epi32 (test_data) ; let reference_result = _mm512_set_epi32 (0 , 1 , 32 , 1 , 3 , 15 , 31 , 28 , 1 , 5 , 6 , 3 , 10 , 17 , 8 , 1) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm512_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_mask_popcnt_epi32 () { let test_data = _mm512_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100 , 0x40_00_00_00 , 103 , 371 , 552 , 432_948 , 818_826_998 , 255 , 256 ,) ; let mask = 0xFF_00 ; let actual_result = _mm512_mask_popcnt_epi32 (test_data , mask , test_data) ; let reference_result = _mm512_set_epi32 (0 , 1 , 32 , 1 , 3 , 15 , 31 , 28 , 0x40_00_00_00 , 103 , 371 , 552 , 432_948 , 818_826_998 , 255 , 256 ,) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm512_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_maskz_popcnt_epi32 () { let test_data = _mm512_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100 , 0x40_00_00_00 , 103 , 371 , 552 , 432_948 , 818_826_998 , 255 , 256 ,) ; let mask = 0xFF_00 ; let actual_result = _mm512_maskz_popcnt_epi32 (mask , test_data) ; let reference_result = _mm512_set_epi32 (0 , 1 , 32 , 1 , 3 , 15 , 31 , 28 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm256_popcnt_epi32 () { let test_data = _mm256_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100) ; let actual_result = _mm256_popcnt_epi32 (test_data) ; let reference_result = _mm256_set_epi32 (0 , 1 , 32 , 1 , 3 , 15 , 31 , 28) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm256_mask_popcnt_epi32 () { let test_data = _mm256_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100) ; let mask = 0xF0 ; let actual_result = _mm256_mask_popcnt_epi32 (test_data , mask , test_data) ; let reference_result = _mm256_set_epi32 (0 , 1 , 32 , 1 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm256_maskz_popcnt_epi32 () { let test_data = _mm256_set_epi32 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF , - 100) ; let mask = 0xF0 ; let actual_result = _mm256_maskz_popcnt_epi32 (mask , test_data) ; let reference_result = _mm256_set_epi32 (0 , 1 , 32 , 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm_popcnt_epi32 () { let test_data = _mm_set_epi32 (0 , 1 , - 1 , - 100) ; let actual_result = _mm_popcnt_epi32 (test_data) ; let reference_result = _mm_set_epi32 (0 , 1 , 32 , 28) ; assert_eq_m128i (actual_result , reference_result) ; }
}

macro_rules! test_mm_mask_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm_mask_popcnt_epi32 () { let test_data = _mm_set_epi32 (0 , 1 , - 1 , - 100) ; let mask = 0xE ; let actual_result = _mm_mask_popcnt_epi32 (test_data , mask , test_data) ; let reference_result = _mm_set_epi32 (0 , 1 , 32 , - 100) ; assert_eq_m128i (actual_result , reference_result) ; }
}

macro_rules! test_mm_maskz_popcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_popcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_popcnt_epi32_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f,avx512vl")] unsafe fn test_mm_maskz_popcnt_epi32 () { let test_data = _mm_set_epi32 (0 , 1 , - 1 , - 100) ; let mask = 0xE ; let actual_result = _mm_maskz_popcnt_epi32 (mask , test_data) ; let reference_result = _mm_set_epi32 (0 , 1 , 32 , 0) ; assert_eq_m128i (actual_result , reference_result) ; }
}

macro_rules! test_mm512_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_popcnt_epi64 () { let test_data = _mm512_set_epi64 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF_FF_FF_FF_FF , - 100) ; let actual_result = _mm512_popcnt_epi64 (test_data) ; let reference_result = _mm512_set_epi64 (0 , 1 , 64 , 1 , 3 , 15 , 63 , 60) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm512_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_mask_popcnt_epi64 () { let test_data = _mm512_set_epi64 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF_FF_FF_FF_FF , - 100) ; let mask = 0xF0 ; let actual_result = _mm512_mask_popcnt_epi64 (test_data , mask , test_data) ; let reference_result = _mm512_set_epi64 (0 , 1 , 64 , 1 , 7 , 0xFF_FE , 0x7F_FF_FF_FF_FF_FF_FF_FF , - 100) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm512_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512f")] unsafe fn test_mm512_maskz_popcnt_epi64 () { let test_data = _mm512_set_epi64 (0 , 1 , - 1 , 2 , 7 , 0xFF_FE , 0x7F_FF_FF_FF_FF_FF_FF_FF , - 100) ; let mask = 0xF0 ; let actual_result = _mm512_maskz_popcnt_epi64 (mask , test_data) ; let reference_result = _mm512_set_epi64 (0 , 1 , 64 , 1 , 0 , 0 , 0 , 0) ; assert_eq_m512i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm256_popcnt_epi64 () { let test_data = _mm256_set_epi64x (0 , 1 , - 1 , - 100) ; let actual_result = _mm256_popcnt_epi64 (test_data) ; let reference_result = _mm256_set_epi64x (0 , 1 , 64 , 60) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm256_mask_popcnt_epi64 () { let test_data = _mm256_set_epi64x (0 , 1 , - 1 , - 100) ; let mask = 0xE ; let actual_result = _mm256_mask_popcnt_epi64 (test_data , mask , test_data) ; let reference_result = _mm256_set_epi64x (0 , 1 , 64 , - 100) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm256_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm256_maskz_popcnt_epi64 () { let test_data = _mm256_set_epi64x (0 , 1 , - 1 , - 100) ; let mask = 0xE ; let actual_result = _mm256_maskz_popcnt_epi64 (mask , test_data) ; let reference_result = _mm256_set_epi64x (0 , 1 , 64 , 0) ; assert_eq_m256i (actual_result , reference_result) ; }
}

macro_rules! test_mm_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm_popcnt_epi64 () { let test_data = _mm_set_epi64x (0 , 1) ; let actual_result = _mm_popcnt_epi64 (test_data) ; let reference_result = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (actual_result , reference_result) ; let test_data = _mm_set_epi64x (- 1 , - 100) ; let actual_result = _mm_popcnt_epi64 (test_data) ; let reference_result = _mm_set_epi64x (64 , 60) ; assert_eq_m128i (actual_result , reference_result) ; }
}

macro_rules! test_mm_mask_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm_mask_popcnt_epi64 () { let test_data = _mm_set_epi64x (0 , - 100) ; let mask = 0x2 ; let actual_result = _mm_mask_popcnt_epi64 (test_data , mask , test_data) ; let reference_result = _mm_set_epi64x (0 , - 100) ; assert_eq_m128i (actual_result , reference_result) ; let test_data = _mm_set_epi64x (- 1 , 1) ; let mask = 0x2 ; let actual_result = _mm_mask_popcnt_epi64 (test_data , mask , test_data) ; let reference_result = _mm_set_epi64x (64 , 1) ; assert_eq_m128i (actual_result , reference_result) ; }
}

macro_rules! test_mm_maskz_popcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_popcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_popcnt_epi64_introspect!();
    # [simd_test (enable = "avx512vpopcntdq,avx512vl")] unsafe fn test_mm_maskz_popcnt_epi64 () { let test_data = _mm_set_epi64x (0 , 1) ; let mask = 0x2 ; let actual_result = _mm_maskz_popcnt_epi64 (mask , test_data) ; let reference_result = _mm_set_epi64x (0 , 0) ; assert_eq_m128i (actual_result , reference_result) ; let test_data = _mm_set_epi64x (- 1 , - 100) ; let mask = 0x2 ; let actual_result = _mm_maskz_popcnt_epi64 (mask , test_data) ; let reference_result = _mm_set_epi64x (64 , 0) ; assert_eq_m128i (actual_result , reference_result) ; }
} 
            }}