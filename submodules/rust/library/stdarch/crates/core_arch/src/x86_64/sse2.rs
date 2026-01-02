mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse2.cvtsd2si64"] fn cvtsd2si64 (a : __m128d) -> i64 ; # [link_name = "llvm.x86.sse2.cvttsd2si64"] fn cvttsd2si64 (a : __m128d) -> i64 ; }}

macro_rules! _mm_cvtsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_si64_introspect!();
    # [doc = " Converts the lower double-precision (64-bit) floating-point element in a to"] # [doc = " a 64-bit integer."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_si64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsd_si64 (a : __m128d) -> i64 { unsafe { cvtsd2si64 (a) } }
}

macro_rules! _mm_cvtsd_si64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_si64x in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_si64x_introspect!();
    # [doc = " Alias for `_mm_cvtsd_si64`"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_si64x)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsd_si64x (a : __m128d) -> i64 { _mm_cvtsd_si64 (a) }
}

macro_rules! _mm_cvttsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttsd_si64_introspect!();
    # [doc = " Converts the lower double-precision (64-bit) floating-point element in `a`"] # [doc = " to a 64-bit integer with truncation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttsd_si64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvttsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttsd_si64 (a : __m128d) -> i64 { unsafe { cvttsd2si64 (a) } }
}

macro_rules! _mm_cvttsd_si64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttsd_si64x in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttsd_si64x_introspect!();
    # [doc = " Alias for `_mm_cvttsd_si64`"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttsd_si64x)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvttsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttsd_si64x (a : __m128d) -> i64 { _mm_cvttsd_si64 (a) }
}

macro_rules! _mm_stream_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_si64_introspect!();
    # [doc = " Stores a 64-bit integer value in the specified memory location."] # [doc = " To minimize caching, the data is flagged as non-temporal (unlikely to be"] # [doc = " used again soon)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_stream_si64)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movnti))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_stream_si64 (mem_addr : * mut i64 , a : i64) { crate :: arch :: asm ! (vps ! ("movnti" , ",{a}") , p = in (reg) mem_addr , a = in (reg) a , options (nostack , preserves_flags) ,) ; }
}

macro_rules! _mm_cvtsi64_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi64_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi64_si128_introspect!();
    # [doc = " Returns a vector whose lowest element is `a` and all higher elements are"] # [doc = " `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi64_si128 (a : i64) -> __m128i { _mm_set_epi64x (0 , a) }
}

macro_rules! _mm_cvtsi64x_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi64x_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi64x_si128_introspect!();
    # [doc = " Returns a vector whose lowest element is `a` and all higher elements are"] # [doc = " `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64x_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi64x_si128 (a : i64) -> __m128i { _mm_cvtsi64_si128 (a) }
}

macro_rules! _mm_cvtsi128_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi128_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi128_si64_introspect!();
    # [doc = " Returns the lowest element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi128_si64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi128_si64 (a : __m128i) -> i64 { unsafe { simd_extract ! (a . as_i64x2 () , 0) } }
}

macro_rules! _mm_cvtsi128_si64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi128_si64x in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi128_si64x_introspect!();
    # [doc = " Returns the lowest element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi128_si64x)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi128_si64x (a : __m128i) -> i64 { _mm_cvtsi128_si64 (a) }
}

macro_rules! _mm_cvtsi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi64_sd_introspect!();
    # [doc = " Returns `a` with its lower element replaced by `b` after converting it to"] # [doc = " an `f64`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsi2sd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi64_sd (a : __m128d , b : i64) -> __m128d { unsafe { simd_insert ! (a , 0 , b as f64) } }
}

macro_rules! _mm_cvtsi64x_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi64x_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi64x_sd_introspect!();
    # [doc = " Returns `a` with its lower element replaced by `b` after converting it to"] # [doc = " an `f64`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64x_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsi2sd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi64x_sd (a : __m128d , b : i64) -> __m128d { _mm_cvtsi64_sd (a , b) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: arch :: x86_64 :: * ;}
mkuse!{use std :: boxed ;}
mkuse!{use std :: ptr ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm_cvtsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_si64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsd_si64 () { let r = _mm_cvtsd_si64 (_mm_setr_pd (- 2.0 , 5.0)) ; assert_eq ! (r , - 2_i64) ; let r = _mm_cvtsd_si64 (_mm_setr_pd (f64 :: MAX , f64 :: MIN)) ; assert_eq ! (r , i64 :: MIN) ; }
}

macro_rules! test_mm_cvtsd_si64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_si64x in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_si64x_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsd_si64x () { let r = _mm_cvtsd_si64x (_mm_setr_pd (f64 :: NAN , f64 :: NAN)) ; assert_eq ! (r , i64 :: MIN) ; }
}

macro_rules! test_mm_cvttsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttsd_si64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvttsd_si64 () { let a = _mm_setr_pd (- 1.1 , 2.2) ; let r = _mm_cvttsd_si64 (a) ; assert_eq ! (r , - 1_i64) ; }
}

macro_rules! test_mm_cvttsd_si64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttsd_si64x in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttsd_si64x_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvttsd_si64x () { let a = _mm_setr_pd (f64 :: NEG_INFINITY , f64 :: NAN) ; let r = _mm_cvttsd_si64x (a) ; assert_eq ! (r , i64 :: MIN) ; }
}

macro_rules! test_mm_stream_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_si64_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_si64 () { let a : i64 = 7 ; let mut mem = boxed :: Box :: < i64 > :: new (- 1) ; _mm_stream_si64 (ptr :: addr_of_mut ! (* mem) , a) ; assert_eq ! (a , * mem) ; }
}

macro_rules! test_mm_cvtsi64_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi64_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi64_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi64_si128 () { let r = _mm_cvtsi64_si128 (5) ; assert_eq_m128i (r , _mm_setr_epi64x (5 , 0)) ; }
}

macro_rules! test_mm_cvtsi128_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi128_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi128_si64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi128_si64 () { let r = _mm_cvtsi128_si64 (_mm_setr_epi64x (5 , 0)) ; assert_eq ! (r , 5) ; }
}

macro_rules! test_mm_cvtsi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi64_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi64_sd () { let a = _mm_set1_pd (3.5) ; let r = _mm_cvtsi64_sd (a , 5) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 3.5)) ; }
} 
            }}