mkuse!{use crate :: core_arch :: x86 :: __m128i ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.pclmulqdq"] fn pclmulqdq (a : __m128i , round_key : __m128i , imm8 : u8) -> __m128i ; }}

macro_rules! _mm_clmulepi64_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_clmulepi64_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_clmulepi64_si128_introspect!();
    # [doc = " Performs a carry-less multiplication of two 64-bit polynomials over the"] # [doc = " finite field GF(2)."] # [doc = ""] # [doc = " The immediate byte is used for determining which halves of `a` and `b`"] # [doc = " should be used. Immediate bits other than 0 and 4 are ignored."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_clmulepi64_si128)"] # [inline] # [target_feature (enable = "pclmulqdq")] # [cfg_attr (test , assert_instr (pclmul , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_clmulepi64_si128 < const IMM8 : i32 > (a : __m128i , b : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { pclmulqdq (a , b , IMM8 as u8) } }
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

macro_rules! test_mm_clmulepi64_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_clmulepi64_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_clmulepi64_si128_introspect!();
    # [simd_test (enable = "pclmulqdq")] unsafe fn test_mm_clmulepi64_si128 () { let a = _mm_set_epi64x (0x7b5b546573745665 , 0x63746f725d53475d) ; let b = _mm_set_epi64x (0x4869285368617929 , 0x5b477565726f6e5d) ; let r00 = _mm_set_epi64x (0x1d4d84c85c3440c0 , 0x929633d5d36f0451) ; let r01 = _mm_set_epi64x (0x1bd17c8d556ab5a1 , 0x7fa540ac2a281315) ; let r10 = _mm_set_epi64x (0x1a2bf6db3a30862f , 0xbabf262df4b7d5c9) ; let r11 = _mm_set_epi64x (0x1d1e1f2c592e7c45 , 0xd66ee03e410fd4ed) ; assert_eq_m128i (_mm_clmulepi64_si128 :: < 0x00 > (a , b) , r00) ; assert_eq_m128i (_mm_clmulepi64_si128 :: < 0x10 > (a , b) , r01) ; assert_eq_m128i (_mm_clmulepi64_si128 :: < 0x01 > (a , b) , r10) ; assert_eq_m128i (_mm_clmulepi64_si128 :: < 0x11 > (a , b) , r11) ; let a0 = _mm_set_epi64x (0x0000000000000000 , 0x8000000000000000) ; let r = _mm_set_epi64x (0x4000000000000000 , 0x0000000000000000) ; assert_eq_m128i (_mm_clmulepi64_si128 :: < 0x00 > (a0 , a0) , r) ; }
} 
            }}