mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse.cvtss2si64"] fn cvtss2si64 (a : __m128) -> i64 ; # [link_name = "llvm.x86.sse.cvttss2si64"] fn cvttss2si64 (a : __m128) -> i64 ; # [link_name = "llvm.x86.sse.cvtsi642ss"] fn cvtsi642ss (a : __m128 , b : i64) -> __m128 ; }}

macro_rules! _mm_cvtss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_si64_introspect!();
    # [doc = " Converts the lowest 32 bit float in the input vector to a 64 bit integer."] # [doc = ""] # [doc = " The result is rounded according to the current rounding mode. If the result"] # [doc = " cannot be represented as a 64 bit integer the result will be"] # [doc = " `0x8000_0000_0000_0000` (`i64::MIN`) or trigger an invalid operation"] # [doc = " floating point exception if unmasked (see"] # [doc = " [`_mm_setcsr`](fn._mm_setcsr.html))."] # [doc = ""] # [doc = " This corresponds to the `CVTSS2SI` instruction (with 64 bit output)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_si64)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtss_si64 (a : __m128) -> i64 { unsafe { cvtss2si64 (a) } }
}

macro_rules! _mm_cvttss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttss_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttss_si64_introspect!();
    # [doc = " Converts the lowest 32 bit float in the input vector to a 64 bit integer"] # [doc = " with truncation."] # [doc = ""] # [doc = " The result is rounded always using truncation (round towards zero). If the"] # [doc = " result cannot be represented as a 64 bit integer the result will be"] # [doc = " `0x8000_0000_0000_0000` (`i64::MIN`) or an invalid operation floating"] # [doc = " point exception if unmasked (see [`_mm_setcsr`](fn._mm_setcsr.html))."] # [doc = ""] # [doc = " This corresponds to the `CVTTSS2SI` instruction (with 64 bit output)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttss_si64)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvttss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttss_si64 (a : __m128) -> i64 { unsafe { cvttss2si64 (a) } }
}

macro_rules! _mm_cvtsi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi64_ss_introspect!();
    # [doc = " Converts a 64 bit integer to a 32 bit float. The result vector is the input"] # [doc = " vector `a` with the lowest 32 bit float replaced by the converted integer."] # [doc = ""] # [doc = " This intrinsic corresponds to the `CVTSI2SS` instruction (with 64 bit"] # [doc = " input)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi64_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtsi2ss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi64_ss (a : __m128 , b : i64) -> __m128 { unsafe { cvtsi642ss (a , b) } }
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

macro_rules! test_mm_cvtss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_si64_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvtss_si64 () { let inputs = & [(42.0f32 , 42i64) , (- 31.4 , - 31) , (- 33.5 , - 34) , (- 34.5 , - 34) , (4.0e10 , 40_000_000_000) , (4.0e-10 , 0) , (f32 :: NAN , i64 :: MIN) , (2147483500.1 , 2147483520) , (9.223371e18 , 9223370937343148032) ,] ; for (i , & (xi , e)) in inputs . iter () . enumerate () { let x = _mm_setr_ps (xi , 1.0 , 3.0 , 4.0) ; let r = _mm_cvtss_si64 (x) ; assert_eq ! (e , r , "TestCase #{} _mm_cvtss_si64({:?}) = {}, expected: {}" , i , x , r , e) ; } }
}

macro_rules! test_mm_cvttss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttss_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttss_si64_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvttss_si64 () { let inputs = & [(42.0f32 , 42i64) , (- 31.4 , - 31) , (- 33.5 , - 33) , (- 34.5 , - 34) , (10.999 , 10) , (- 5.99 , - 5) , (4.0e10 , 40_000_000_000) , (4.0e-10 , 0) , (f32 :: NAN , i64 :: MIN) , (2147483500.1 , 2147483520) , (9.223371e18 , 9223370937343148032) , (9.223372e18 , i64 :: MIN) ,] ; for (i , & (xi , e)) in inputs . iter () . enumerate () { let x = _mm_setr_ps (xi , 1.0 , 3.0 , 4.0) ; let r = _mm_cvttss_si64 (x) ; assert_eq ! (e , r , "TestCase #{} _mm_cvttss_si64({:?}) = {}, expected: {}" , i , x , r , e) ; } }
}

macro_rules! test_mm_cvtsi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi64_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvtsi64_ss () { let inputs = & [(4555i64 , 4555.0f32) , (322223333 , 322223330.0) , (- 432 , - 432.0) , (- 322223333 , - 322223330.0) , (9223372036854775807 , 9.223372e18) , (- 9223372036854775808 , - 9.223372e18) ,] ; for & (x , f) in inputs { let a = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_cvtsi64_ss (a , x) ; let e = _mm_setr_ps (f , 6.0 , 7.0 , 8.0) ; assert_eq_m128 (e , r) ; } }
} 
            }}