mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use crate :: intrinsics :: simd :: simd_select_bitmask ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm512_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm512_madd52hi_epu64 (a : __m512i , b : __m512i , c : __m512i) -> __m512i { unsafe { vpmadd52huq_512 (a , b , c) } }
}

macro_rules! _mm512_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_mask_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm512_mask_madd52hi_epu64 (a : __m512i , k : __mmask8 , b : __m512i , c : __m512i) -> __m512i { unsafe { simd_select_bitmask (k , vpmadd52huq_512 (a , b , c) , a) } }
}

macro_rules! _mm512_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_maskz_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm512_maskz_madd52hi_epu64 (k : __mmask8 , a : __m512i , b : __m512i , c : __m512i) -> __m512i { unsafe { simd_select_bitmask (k , vpmadd52huq_512 (a , b , c) , _mm512_setzero_si512 ()) } }
}

macro_rules! _mm512_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm512_madd52lo_epu64 (a : __m512i , b : __m512i , c : __m512i) -> __m512i { unsafe { vpmadd52luq_512 (a , b , c) } }
}

macro_rules! _mm512_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_mask_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm512_mask_madd52lo_epu64 (a : __m512i , k : __mmask8 , b : __m512i , c : __m512i) -> __m512i { unsafe { simd_select_bitmask (k , vpmadd52luq_512 (a , b , c) , a) } }
}

macro_rules! _mm512_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm512_maskz_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm512_maskz_madd52lo_epu64 (k : __mmask8 , a : __m512i , b : __m512i , c : __m512i) -> __m512i { unsafe { simd_select_bitmask (k , vpmadd52luq_512 (a , b , c) , _mm512_setzero_si512 ()) } }
}

macro_rules! _mm256_madd52hi_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_madd52hi_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_madd52hi_avx_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_madd52hi_avx_epu64)"] # [inline] # [target_feature (enable = "avxifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm256_madd52hi_avx_epu64 (a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { vpmadd52huq_256 (a , b , c) } }
}

macro_rules! _mm256_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm256_madd52hi_epu64 (a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { vpmadd52huq_256 (a , b , c) } }
}

macro_rules! _mm256_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_mask_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm256_mask_madd52hi_epu64 (a : __m256i , k : __mmask8 , b : __m256i , c : __m256i) -> __m256i { unsafe { simd_select_bitmask (k , vpmadd52huq_256 (a , b , c) , a) } }
}

macro_rules! _mm256_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_maskz_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm256_maskz_madd52hi_epu64 (k : __mmask8 , a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { simd_select_bitmask (k , vpmadd52huq_256 (a , b , c) , _mm256_setzero_si256 ()) } }
}

macro_rules! _mm256_madd52lo_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_madd52lo_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_madd52lo_avx_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_madd52lo_avx_epu64)"] # [inline] # [target_feature (enable = "avxifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm256_madd52lo_avx_epu64 (a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { vpmadd52luq_256 (a , b , c) } }
}

macro_rules! _mm256_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm256_madd52lo_epu64 (a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { vpmadd52luq_256 (a , b , c) } }
}

macro_rules! _mm256_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_mask_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm256_mask_madd52lo_epu64 (a : __m256i , k : __mmask8 , b : __m256i , c : __m256i) -> __m256i { unsafe { simd_select_bitmask (k , vpmadd52luq_256 (a , b , c) , a) } }
}

macro_rules! _mm256_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm256_maskz_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm256_maskz_madd52lo_epu64 (k : __mmask8 , a : __m256i , b : __m256i , c : __m256i) -> __m256i { unsafe { simd_select_bitmask (k , vpmadd52luq_256 (a , b , c) , _mm256_setzero_si256 ()) } }
}

macro_rules! _mm_madd52hi_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_madd52hi_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_madd52hi_avx_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_madd52hi_avx_epu64)"] # [inline] # [target_feature (enable = "avxifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm_madd52hi_avx_epu64 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { vpmadd52huq_128 (a , b , c) } }
}

macro_rules! _mm_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm_madd52hi_epu64 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { vpmadd52huq_128 (a , b , c) } }
}

macro_rules! _mm_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_mask_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm_mask_madd52hi_epu64 (a : __m128i , k : __mmask8 , b : __m128i , c : __m128i) -> __m128i { unsafe { simd_select_bitmask (k , vpmadd52huq_128 (a , b , c) , a) } }
}

macro_rules! _mm_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_madd52hi_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the high 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_maskz_madd52hi_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52huq))] pub fn _mm_maskz_madd52hi_epu64 (k : __mmask8 , a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { simd_select_bitmask (k , vpmadd52huq_128 (a , b , c) , _mm_setzero_si128 ()) } }
}

macro_rules! _mm_madd52lo_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_madd52lo_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_madd52lo_avx_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_madd52lo_avx_epu64)"] # [inline] # [target_feature (enable = "avxifma")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm_madd52lo_avx_epu64 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { vpmadd52luq_128 (a , b , c) } }
}

macro_rules! _mm_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm_madd52lo_epu64 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { vpmadd52luq_128 (a , b , c) } }
}

macro_rules! _mm_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are copied"] # [doc = " from `k` when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_mask_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm_mask_madd52lo_epu64 (a : __m128i , k : __mmask8 , b : __m128i , c : __m128i) -> __m128i { unsafe { simd_select_bitmask (k , vpmadd52luq_128 (a , b , c) , a) } }
}

macro_rules! _mm_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_madd52lo_epu64_introspect!();
    # [doc = " Multiply packed unsigned 52-bit integers in each 64-bit element of"] # [doc = " `b` and `c` to form a 104-bit intermediate result. Add the low 52-bit"] # [doc = " unsigned integer from the intermediate result with the"] # [doc = " corresponding unsigned 64-bit integer in `a`, and store the"] # [doc = " results in `dst` using writemask `k` (elements are zeroed"] # [doc = " out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#avx512techs=AVX512IFMA52&text=_mm_maskz_madd52lo_epu64)"] # [inline] # [target_feature (enable = "avx512ifma,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpmadd52luq))] pub fn _mm_maskz_madd52lo_epu64 (k : __mmask8 , a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { simd_select_bitmask (k , vpmadd52luq_128 (a , b , c) , _mm_setzero_si128 ()) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.avx512.vpmadd52l.uq.128"] fn vpmadd52luq_128 (z : __m128i , x : __m128i , y : __m128i) -> __m128i ; # [link_name = "llvm.x86.avx512.vpmadd52h.uq.128"] fn vpmadd52huq_128 (z : __m128i , x : __m128i , y : __m128i) -> __m128i ; # [link_name = "llvm.x86.avx512.vpmadd52l.uq.256"] fn vpmadd52luq_256 (z : __m256i , x : __m256i , y : __m256i) -> __m256i ; # [link_name = "llvm.x86.avx512.vpmadd52h.uq.256"] fn vpmadd52huq_256 (z : __m256i , x : __m256i , y : __m256i) -> __m256i ; # [link_name = "llvm.x86.avx512.vpmadd52l.uq.512"] fn vpmadd52luq_512 (z : __m512i , x : __m512i , y : __m512i) -> __m512i ; # [link_name = "llvm.x86.avx512.vpmadd52h.uq.512"] fn vpmadd52huq_512 (z : __m512i , x : __m512i , y : __m512i) -> __m512i ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkitem!{const K : __mmask8 = 0b01101101 ;}

macro_rules! test_mm512_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_madd52hi_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_madd52hi_epu64 (a , b , c) ; let expected = _mm512_set1_epi64 (11030549757952) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm512_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_mask_madd52hi_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_mask_madd52hi_epu64 (a , K , b , c) ; let mut expected = _mm512_set1_epi64 (11030549757952) ; expected = _mm512_mask_blend_epi64 (K , a , expected) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm512_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_maskz_madd52hi_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_maskz_madd52hi_epu64 (K , a , b , c) ; let mut expected = _mm512_set1_epi64 (11030549757952) ; expected = _mm512_mask_blend_epi64 (K , _mm512_setzero_si512 () , expected) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm512_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_madd52lo_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_madd52lo_epu64 (a , b , c) ; let expected = _mm512_set1_epi64 (100055558127628) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm512_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_mask_madd52lo_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_mask_madd52lo_epu64 (a , K , b , c) ; let mut expected = _mm512_set1_epi64 (100055558127628) ; expected = _mm512_mask_blend_epi64 (K , a , expected) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm512_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma")] unsafe fn test_mm512_maskz_madd52lo_epu64 () { let a = _mm512_set1_epi64 (10 << 40) ; let b = _mm512_set1_epi64 ((11 << 40) + 4) ; let c = _mm512_set1_epi64 ((12 << 40) + 3) ; let actual = _mm512_maskz_madd52lo_epu64 (K , a , b , c) ; let mut expected = _mm512_set1_epi64 (100055558127628) ; expected = _mm512_mask_blend_epi64 (K , _mm512_setzero_si512 () , expected) ; assert_eq_m512i (expected , actual) ; }
}

macro_rules! test_mm256_madd52hi_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_madd52hi_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_madd52hi_avx_epu64_introspect!();
    # [simd_test (enable = "avxifma")] unsafe fn test_mm256_madd52hi_avx_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_madd52hi_avx_epu64 (a , b , c) ; let expected = _mm256_set1_epi64x (11030549757952) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_madd52hi_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_madd52hi_epu64 (a , b , c) ; let expected = _mm256_set1_epi64x (11030549757952) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_mask_madd52hi_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_mask_madd52hi_epu64 (a , K , b , c) ; let mut expected = _mm256_set1_epi64x (11030549757952) ; expected = _mm256_mask_blend_epi64 (K , a , expected) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_maskz_madd52hi_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_maskz_madd52hi_epu64 (K , a , b , c) ; let mut expected = _mm256_set1_epi64x (11030549757952) ; expected = _mm256_mask_blend_epi64 (K , _mm256_setzero_si256 () , expected) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_madd52lo_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_madd52lo_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_madd52lo_avx_epu64_introspect!();
    # [simd_test (enable = "avxifma")] unsafe fn test_mm256_madd52lo_avx_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_madd52lo_avx_epu64 (a , b , c) ; let expected = _mm256_set1_epi64x (100055558127628) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_madd52lo_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_madd52lo_epu64 (a , b , c) ; let expected = _mm256_set1_epi64x (100055558127628) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_mask_madd52lo_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_mask_madd52lo_epu64 (a , K , b , c) ; let mut expected = _mm256_set1_epi64x (100055558127628) ; expected = _mm256_mask_blend_epi64 (K , a , expected) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm256_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm256_maskz_madd52lo_epu64 () { let a = _mm256_set1_epi64x (10 << 40) ; let b = _mm256_set1_epi64x ((11 << 40) + 4) ; let c = _mm256_set1_epi64x ((12 << 40) + 3) ; let actual = _mm256_maskz_madd52lo_epu64 (K , a , b , c) ; let mut expected = _mm256_set1_epi64x (100055558127628) ; expected = _mm256_mask_blend_epi64 (K , _mm256_setzero_si256 () , expected) ; assert_eq_m256i (expected , actual) ; }
}

macro_rules! test_mm_madd52hi_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_madd52hi_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_madd52hi_avx_epu64_introspect!();
    # [simd_test (enable = "avxifma")] unsafe fn test_mm_madd52hi_avx_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_madd52hi_avx_epu64 (a , b , c) ; let expected = _mm_set1_epi64x (11030549757952) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_madd52hi_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_madd52hi_epu64 (a , b , c) ; let expected = _mm_set1_epi64x (11030549757952) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_mask_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_mask_madd52hi_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_mask_madd52hi_epu64 (a , K , b , c) ; let mut expected = _mm_set1_epi64x (11030549757952) ; expected = _mm_mask_blend_epi64 (K , a , expected) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_maskz_madd52hi_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_madd52hi_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_madd52hi_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_maskz_madd52hi_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_maskz_madd52hi_epu64 (K , a , b , c) ; let mut expected = _mm_set1_epi64x (11030549757952) ; expected = _mm_mask_blend_epi64 (K , _mm_setzero_si128 () , expected) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_madd52lo_avx_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_madd52lo_avx_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_madd52lo_avx_epu64_introspect!();
    # [simd_test (enable = "avxifma")] unsafe fn test_mm_madd52lo_avx_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_madd52lo_avx_epu64 (a , b , c) ; let expected = _mm_set1_epi64x (100055558127628) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_madd52lo_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_madd52lo_epu64 (a , b , c) ; let expected = _mm_set1_epi64x (100055558127628) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_mask_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_mask_madd52lo_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_mask_madd52lo_epu64 (a , K , b , c) ; let mut expected = _mm_set1_epi64x (100055558127628) ; expected = _mm_mask_blend_epi64 (K , a , expected) ; assert_eq_m128i (expected , actual) ; }
}

macro_rules! test_mm_maskz_madd52lo_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_madd52lo_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_madd52lo_epu64_introspect!();
    # [simd_test (enable = "avx512ifma,avx512vl")] unsafe fn test_mm_maskz_madd52lo_epu64 () { let a = _mm_set1_epi64x (10 << 40) ; let b = _mm_set1_epi64x ((11 << 40) + 4) ; let c = _mm_set1_epi64x ((12 << 40) + 3) ; let actual = _mm_maskz_madd52lo_epu64 (K , a , b , c) ; let mut expected = _mm_set1_epi64x (100055558127628) ; expected = _mm_mask_blend_epi64 (K , _mm_setzero_si128 () , expected) ; assert_eq_m128i (expected , actual) ; }
} 
            }}