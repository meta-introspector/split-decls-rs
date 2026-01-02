mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.x86.vcvtph2ps.128"] fn llvm_vcvtph2ps_128 (a : i16x8) -> f32x4 ; # [link_name = "llvm.x86.vcvtph2ps.256"] fn llvm_vcvtph2ps_256 (a : i16x8) -> f32x8 ; # [link_name = "llvm.x86.vcvtps2ph.128"] fn llvm_vcvtps2ph_128 (a : f32x4 , rounding : i32) -> i16x8 ; # [link_name = "llvm.x86.vcvtps2ph.256"] fn llvm_vcvtps2ph_256 (a : f32x8 , rounding : i32) -> i16x8 ; }}

macro_rules! _mm_cvtph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtph_ps_introspect!();
    # [doc = " Converts the 4 x 16-bit half-precision float values in the lowest 64-bit of"] # [doc = " the 128-bit vector `a` into 4 x 32-bit float values stored in a 128-bit wide"] # [doc = " vector."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtph_ps)"] # [inline] # [target_feature (enable = "f16c")] # [cfg_attr (test , assert_instr ("vcvtph2ps"))] # [stable (feature = "x86_f16c_intrinsics" , since = "1.68.0")] pub fn _mm_cvtph_ps (a : __m128i) -> __m128 { unsafe { transmute (llvm_vcvtph2ps_128 (transmute (a))) } }
}

macro_rules! _mm256_cvtph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtph_ps_introspect!();
    # [doc = " Converts the 8 x 16-bit half-precision float values in the 128-bit vector"] # [doc = " `a` into 8 x 32-bit float values stored in a 256-bit wide vector."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtph_ps)"] # [inline] # [target_feature (enable = "f16c")] # [cfg_attr (test , assert_instr ("vcvtph2ps"))] # [stable (feature = "x86_f16c_intrinsics" , since = "1.68.0")] pub fn _mm256_cvtph_ps (a : __m128i) -> __m256 { unsafe { transmute (llvm_vcvtph2ps_256 (transmute (a))) } }
}

macro_rules! _mm_cvtps_ph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtps_ph in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtps_ph_introspect!();
    # [doc = " Converts the 4 x 32-bit float values in the 128-bit vector `a` into 4 x"] # [doc = " 16-bit half-precision float values stored in the lowest 64-bit of a 128-bit"] # [doc = " vector."] # [doc = ""] # [doc = " Rounding is done according to the `imm_rounding` parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtps_ph)"] # [inline] # [target_feature (enable = "f16c")] # [cfg_attr (test , assert_instr ("vcvtps2ph" , IMM_ROUNDING = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "x86_f16c_intrinsics" , since = "1.68.0")] pub fn _mm_cvtps_ph < const IMM_ROUNDING : i32 > (a : __m128) -> __m128i { static_assert_uimm_bits ! (IMM_ROUNDING , 3) ; unsafe { let a = a . as_f32x4 () ; let r = llvm_vcvtps2ph_128 (a , IMM_ROUNDING) ; transmute (r) } }
}

macro_rules! _mm256_cvtps_ph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtps_ph in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtps_ph_introspect!();
    # [doc = " Converts the 8 x 32-bit float values in the 256-bit vector `a` into 8 x"] # [doc = " 16-bit half-precision float values stored in a 128-bit wide vector."] # [doc = ""] # [doc = " Rounding is done according to the `imm_rounding` parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtps_ph)"] # [inline] # [target_feature (enable = "f16c")] # [cfg_attr (test , assert_instr ("vcvtps2ph" , IMM_ROUNDING = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "x86_f16c_intrinsics" , since = "1.68.0")] pub fn _mm256_cvtps_ph < const IMM_ROUNDING : i32 > (a : __m256) -> __m128i { static_assert_uimm_bits ! (IMM_ROUNDING , 3) ; unsafe { let a = a . as_f32x8 () ; let r = llvm_vcvtps2ph_256 (a , IMM_ROUNDING) ; transmute (r) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: { core_arch :: x86 :: * , mem :: transmute } ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{const F16_ONE : i16 = 0x3c00 ;}
mkitem!{const F16_TWO : i16 = 0x4000 ;}
mkitem!{const F16_THREE : i16 = 0x4200 ;}
mkitem!{const F16_FOUR : i16 = 0x4400 ;}
mkitem!{const F16_FIVE : i16 = 0x4500 ;}
mkitem!{const F16_SIX : i16 = 0x4600 ;}
mkitem!{const F16_SEVEN : i16 = 0x4700 ;}
mkitem!{const F16_EIGHT : i16 = 0x4800 ;}

macro_rules! test_mm_cvtph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtph_ps_introspect!();
    # [simd_test (enable = "f16c")] unsafe fn test_mm_cvtph_ps () { let a = _mm_set_epi16 (0 , 0 , 0 , 0 , F16_ONE , F16_TWO , F16_THREE , F16_FOUR) ; let r = _mm_cvtph_ps (a) ; let e = _mm_set_ps (1.0 , 2.0 , 3.0 , 4.0) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtph_ps_introspect!();
    # [simd_test (enable = "f16c")] unsafe fn test_mm256_cvtph_ps () { let a = _mm_set_epi16 (F16_ONE , F16_TWO , F16_THREE , F16_FOUR , F16_FIVE , F16_SIX , F16_SEVEN , F16_EIGHT ,) ; let r = _mm256_cvtph_ps (a) ; let e = _mm256_set_ps (1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtps_ph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtps_ph in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtps_ph_introspect!();
    # [simd_test (enable = "f16c")] unsafe fn test_mm_cvtps_ph () { let a = _mm_set_ps (1.0 , 2.0 , 3.0 , 4.0) ; let r = _mm_cvtps_ph :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , F16_ONE , F16_TWO , F16_THREE , F16_FOUR) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtps_ph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtps_ph in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtps_ph_introspect!();
    # [simd_test (enable = "f16c")] unsafe fn test_mm256_cvtps_ph () { let a = _mm256_set_ps (1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm256_cvtps_ph :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm_set_epi16 (F16_ONE , F16_TWO , F16_THREE , F16_FOUR , F16_FIVE , F16_SIX , F16_SEVEN , F16_EIGHT ,) ; assert_eq_m128i (r , e) ; }
} 
            }}