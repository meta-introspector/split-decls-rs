mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , intrinsics :: simd :: * , } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_abs_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_abs_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_abs_epi8_introspect!();
    # [doc = " Computes the absolute value of packed 8-bit signed integers in `a` and"] # [doc = " return the unsigned results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_abs_epi8)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pabsb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_abs_epi8 (a : __m128i) -> __m128i { unsafe { let a = a . as_i8x16 () ; let zero = i8x16 :: ZERO ; let r = simd_select :: < m8x16 , _ > (simd_lt (a , zero) , simd_neg (a) , a) ; transmute (r) } }
}

macro_rules! _mm_abs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_abs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_abs_epi16_introspect!();
    # [doc = " Computes the absolute value of each of the packed 16-bit signed integers in"] # [doc = " `a` and"] # [doc = " return the 16-bit unsigned integer"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_abs_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pabsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_abs_epi16 (a : __m128i) -> __m128i { unsafe { let a = a . as_i16x8 () ; let zero = i16x8 :: ZERO ; let r = simd_select :: < m16x8 , _ > (simd_lt (a , zero) , simd_neg (a) , a) ; transmute (r) } }
}

macro_rules! _mm_abs_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_abs_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_abs_epi32_introspect!();
    # [doc = " Computes the absolute value of each of the packed 32-bit signed integers in"] # [doc = " `a` and"] # [doc = " return the 32-bit unsigned integer"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_abs_epi32)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pabsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_abs_epi32 (a : __m128i) -> __m128i { unsafe { let a = a . as_i32x4 () ; let zero = i32x4 :: ZERO ; let r = simd_select :: < m32x4 , _ > (simd_lt (a , zero) , simd_neg (a) , a) ; transmute (r) } }
}

macro_rules! _mm_shuffle_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shuffle_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_shuffle_epi8_introspect!();
    # [doc = " Shuffles bytes from `a` according to the content of `b`."] # [doc = ""] # [doc = " The last 4 bits of each byte of `b` are used as addresses"] # [doc = " into the 16 bytes of `a`."] # [doc = ""] # [doc = " In addition, if the highest significant bit of a byte of `b`"] # [doc = " is set, the respective destination byte is set to 0."] # [doc = ""] # [doc = " Picturing `a` and `b` as `[u8; 16]`, `_mm_shuffle_epi8` is"] # [doc = " logically equivalent to:"] # [doc = ""] # [doc = " ```"] # [doc = " fn mm_shuffle_epi8(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {"] # [doc = "     let mut r = [0u8; 16];"] # [doc = "     for i in 0..16 {"] # [doc = "         // if the most significant bit of b is set,"] # [doc = "         // then the destination byte is set to 0."] # [doc = "         if b[i] & 0x80 == 0u8 {"] # [doc = "             r[i] = a[(b[i] % 16) as usize];"] # [doc = "         }"] # [doc = "     }"] # [doc = "     r"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shuffle_epi8)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pshufb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shuffle_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (pshufb128 (a . as_u8x16 () , b . as_u8x16 ())) } }
}

macro_rules! _mm_alignr_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_alignr_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_alignr_epi8_introspect!();
    # [doc = " Concatenate 16-byte blocks in `a` and `b` into a 32-byte temporary result,"] # [doc = " shift the result right by `n` bytes, and returns the low 16 bytes."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_alignr_epi8)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (palignr , IMM8 = 15))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_alignr_epi8 < const IMM8 : i32 > (a : __m128i , b : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; if IMM8 > 32 { return _mm_setzero_si128 () ; } let (a , b) = if IMM8 > 16 { (_mm_setzero_si128 () , a) } else { (a , b) } ; const fn mask (shift : u32 , i : u32) -> u32 { if shift > 32 { i } else if shift > 16 { shift - 16 + i } else { shift + i } } unsafe { let r : i8x16 = simd_shuffle ! (b . as_i8x16 () , a . as_i8x16 () , [mask (IMM8 as u32 , 0) , mask (IMM8 as u32 , 1) , mask (IMM8 as u32 , 2) , mask (IMM8 as u32 , 3) , mask (IMM8 as u32 , 4) , mask (IMM8 as u32 , 5) , mask (IMM8 as u32 , 6) , mask (IMM8 as u32 , 7) , mask (IMM8 as u32 , 8) , mask (IMM8 as u32 , 9) , mask (IMM8 as u32 , 10) , mask (IMM8 as u32 , 11) , mask (IMM8 as u32 , 12) , mask (IMM8 as u32 , 13) , mask (IMM8 as u32 , 14) , mask (IMM8 as u32 , 15) ,] ,) ; transmute (r) } }
}

macro_rules! _mm_hadd_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hadd_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hadd_epi16_introspect!();
    # [doc = " Horizontally adds the adjacent pairs of values contained in 2 packed"] # [doc = " 128-bit vectors of `[8 x i16]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hadd_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phaddw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hadd_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phaddw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_hadds_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hadds_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hadds_epi16_introspect!();
    # [doc = " Horizontally adds the adjacent pairs of values contained in 2 packed"] # [doc = " 128-bit vectors of `[8 x i16]`. Positive sums greater than 7FFFh are"] # [doc = " saturated to 7FFFh. Negative sums less than 8000h are saturated to 8000h."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hadds_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phaddsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hadds_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phaddsw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_hadd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hadd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hadd_epi32_introspect!();
    # [doc = " Horizontally adds the adjacent pairs of values contained in 2 packed"] # [doc = " 128-bit vectors of `[4 x i32]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hadd_epi32)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phaddd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hadd_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phaddd128 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_hsub_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hsub_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hsub_epi16_introspect!();
    # [doc = " Horizontally subtract the adjacent pairs of values contained in 2"] # [doc = " packed 128-bit vectors of `[8 x i16]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hsub_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phsubw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hsub_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phsubw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_hsubs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hsubs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hsubs_epi16_introspect!();
    # [doc = " Horizontally subtract the adjacent pairs of values contained in 2"] # [doc = " packed 128-bit vectors of `[8 x i16]`. Positive differences greater than"] # [doc = " 7FFFh are saturated to 7FFFh. Negative differences less than 8000h are"] # [doc = " saturated to 8000h."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hsubs_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phsubsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hsubs_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phsubsw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_hsub_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hsub_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_hsub_epi32_introspect!();
    # [doc = " Horizontally subtract the adjacent pairs of values contained in 2"] # [doc = " packed 128-bit vectors of `[4 x i32]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hsub_epi32)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (phsubd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hsub_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (phsubd128 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_maddubs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maddubs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maddubs_epi16_introspect!();
    # [doc = " Multiplies corresponding pairs of packed 8-bit unsigned integer"] # [doc = " values contained in the first source operand and packed 8-bit signed"] # [doc = " integer values contained in the second source operand, add pairs of"] # [doc = " contiguous products with signed saturation, and writes the 16-bit sums to"] # [doc = " the corresponding bits in the destination."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maddubs_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pmaddubsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_maddubs_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (pmaddubsw128 (a . as_u8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_mulhrs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mulhrs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mulhrs_epi16_introspect!();
    # [doc = " Multiplies packed 16-bit signed integer values, truncate the 32-bit"] # [doc = " product to the 18 most significant bits by right-shifting, round the"] # [doc = " truncated value by adding 1, and write bits `[16:1]` to the destination."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mulhrs_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (pmulhrsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mulhrs_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (pmulhrsw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_sign_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sign_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sign_epi8_introspect!();
    # [doc = " Negates packed 8-bit integers in `a` when the corresponding signed 8-bit"] # [doc = " integer in `b` is negative, and returns the result."] # [doc = " Elements in result are zeroed out when the corresponding element in `b`"] # [doc = " is zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sign_epi8)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (psignb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sign_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (psignb128 (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_sign_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sign_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sign_epi16_introspect!();
    # [doc = " Negates packed 16-bit integers in `a` when the corresponding signed 16-bit"] # [doc = " integer in `b` is negative, and returns the results."] # [doc = " Elements in result are zeroed out when the corresponding element in `b`"] # [doc = " is zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sign_epi16)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (psignw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sign_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (psignw128 (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_sign_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sign_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sign_epi32_introspect!();
    # [doc = " Negates packed 32-bit integers in `a` when the corresponding signed 32-bit"] # [doc = " integer in `b` is negative, and returns the results."] # [doc = " Element in result are zeroed out when the corresponding element in `b`"] # [doc = " is zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sign_epi32)"] # [inline] # [target_feature (enable = "ssse3")] # [cfg_attr (test , assert_instr (psignd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sign_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (psignd128 (a . as_i32x4 () , b . as_i32x4 ())) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.ssse3.pshuf.b.128"] fn pshufb128 (a : u8x16 , b : u8x16) -> u8x16 ; # [link_name = "llvm.x86.ssse3.phadd.w.128"] fn phaddw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.phadd.sw.128"] fn phaddsw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.phadd.d.128"] fn phaddd128 (a : i32x4 , b : i32x4) -> i32x4 ; # [link_name = "llvm.x86.ssse3.phsub.w.128"] fn phsubw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.phsub.sw.128"] fn phsubsw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.phsub.d.128"] fn phsubd128 (a : i32x4 , b : i32x4) -> i32x4 ; # [link_name = "llvm.x86.ssse3.pmadd.ub.sw.128"] fn pmaddubsw128 (a : u8x16 , b : i8x16) -> i16x8 ; # [link_name = "llvm.x86.ssse3.pmul.hr.sw.128"] fn pmulhrsw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.psign.b.128"] fn psignb128 (a : i8x16 , b : i8x16) -> i8x16 ; # [link_name = "llvm.x86.ssse3.psign.w.128"] fn psignw128 (a : i16x8 , b : i16x8) -> i16x8 ; # [link_name = "llvm.x86.ssse3.psign.d.128"] fn psignd128 (a : i32x4 , b : i32x4) -> i32x4 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_mm_abs_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_abs_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_abs_epi8_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_abs_epi8 () { let r = _mm_abs_epi8 (_mm_set1_epi8 (- 5)) ; assert_eq_m128i (r , _mm_set1_epi8 (5)) ; }
}

macro_rules! test_mm_abs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_abs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_abs_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_abs_epi16 () { let r = _mm_abs_epi16 (_mm_set1_epi16 (- 5)) ; assert_eq_m128i (r , _mm_set1_epi16 (5)) ; }
}

macro_rules! test_mm_abs_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_abs_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_abs_epi32_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_abs_epi32 () { let r = _mm_abs_epi32 (_mm_set1_epi32 (- 5)) ; assert_eq_m128i (r , _mm_set1_epi32 (5)) ; }
}

macro_rules! test_mm_shuffle_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shuffle_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shuffle_epi8_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_shuffle_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (4 , 128_u8 as i8 , 4 , 3 , 24 , 12 , 6 , 19 , 12 , 5 , 5 , 10 , 4 , 1 , 8 , 0 ,) ; let expected = _mm_setr_epi8 (5 , 0 , 5 , 4 , 9 , 13 , 7 , 4 , 13 , 6 , 6 , 11 , 5 , 2 , 9 , 1) ; let r = _mm_shuffle_epi8 (a , b) ; assert_eq_m128i (r , expected) ; let b = _mm_add_epi8 (b , _mm_set1_epi8 (32)) ; let r = _mm_shuffle_epi8 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_alignr_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_alignr_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_alignr_epi8_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_alignr_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (4 , 63 , 4 , 3 , 24 , 12 , 6 , 19 , 12 , 5 , 5 , 10 , 4 , 1 , 8 , 0 ,) ; let r = _mm_alignr_epi8 :: < 33 > (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (0)) ; let r = _mm_alignr_epi8 :: < 17 > (a , b) ; # [rustfmt :: skip] let expected = _mm_setr_epi8 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 0 ,) ; assert_eq_m128i (r , expected) ; let r = _mm_alignr_epi8 :: < 16 > (a , b) ; assert_eq_m128i (r , a) ; let r = _mm_alignr_epi8 :: < 15 > (a , b) ; # [rustfmt :: skip] let expected = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; assert_eq_m128i (r , expected) ; let r = _mm_alignr_epi8 :: < 0 > (a , b) ; assert_eq_m128i (r , b) ; }
}

macro_rules! test_mm_hadd_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hadd_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hadd_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hadd_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 4 , 3 , 24 , 12 , 6 , 19) ; let expected = _mm_setr_epi16 (3 , 7 , 11 , 15 , 132 , 7 , 36 , 25) ; let r = _mm_hadd_epi16 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi16 (i16 :: MAX , 1 , i16 :: MAX , 2 , i16 :: MAX , 3 , i16 :: MAX , 4) ; let b = _mm_setr_epi16 (i16 :: MIN , - 1 , i16 :: MIN , - 2 , i16 :: MIN , - 3 , i16 :: MIN , - 4) ; let expected = _mm_setr_epi16 (i16 :: MIN , i16 :: MIN + 1 , i16 :: MIN + 2 , i16 :: MIN + 3 , i16 :: MAX , i16 :: MAX - 1 , i16 :: MAX - 2 , i16 :: MAX - 3 ,) ; let r = _mm_hadd_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_hadds_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hadds_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hadds_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hadds_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 4 , 3 , 32767 , 1 , - 32768 , - 1) ; let expected = _mm_setr_epi16 (3 , 7 , 11 , 15 , 132 , 7 , 32767 , - 32768) ; let r = _mm_hadds_epi16 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi16 (i16 :: MAX , 1 , i16 :: MAX , 2 , i16 :: MAX , 3 , i16 :: MAX , 4) ; let b = _mm_setr_epi16 (i16 :: MIN , - 1 , i16 :: MIN , - 2 , i16 :: MIN , - 3 , i16 :: MIN , - 4) ; let expected = _mm_setr_epi16 (i16 :: MAX , i16 :: MAX , i16 :: MAX , i16 :: MAX , i16 :: MIN , i16 :: MIN , i16 :: MIN , i16 :: MIN ,) ; let r = _mm_hadds_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_hadd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hadd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hadd_epi32_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hadd_epi32 () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let b = _mm_setr_epi32 (4 , 128 , 4 , 3) ; let expected = _mm_setr_epi32 (3 , 7 , 132 , 7) ; let r = _mm_hadd_epi32 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi32 (i32 :: MAX , 1 , i32 :: MAX , 2) ; let b = _mm_setr_epi32 (i32 :: MIN , - 1 , i32 :: MIN , - 2) ; let expected = _mm_setr_epi32 (i32 :: MIN , i32 :: MIN + 1 , i32 :: MAX , i32 :: MAX - 1) ; let r = _mm_hadd_epi32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_hsub_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hsub_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hsub_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hsub_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 4 , 3 , 24 , 12 , 6 , 19) ; let expected = _mm_setr_epi16 (- 1 , - 1 , - 1 , - 1 , - 124 , 1 , 12 , - 13) ; let r = _mm_hsub_epi16 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi16 (i16 :: MAX , - 1 , i16 :: MAX , - 2 , i16 :: MAX , - 3 , i16 :: MAX , - 4) ; let b = _mm_setr_epi16 (i16 :: MIN , 1 , i16 :: MIN , 2 , i16 :: MIN , 3 , i16 :: MIN , 4) ; let expected = _mm_setr_epi16 (i16 :: MIN , i16 :: MIN + 1 , i16 :: MIN + 2 , i16 :: MIN + 3 , i16 :: MAX , i16 :: MAX - 1 , i16 :: MAX - 2 , i16 :: MAX - 3 ,) ; let r = _mm_hsub_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_hsubs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hsubs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hsubs_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hsubs_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 4 , 3 , 32767 , - 1 , - 32768 , 1) ; let expected = _mm_setr_epi16 (- 1 , - 1 , - 1 , - 1 , - 124 , 1 , 32767 , - 32768) ; let r = _mm_hsubs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi16 (i16 :: MAX , - 1 , i16 :: MAX , - 2 , i16 :: MAX , - 3 , i16 :: MAX , - 4) ; let b = _mm_setr_epi16 (i16 :: MIN , 1 , i16 :: MIN , 2 , i16 :: MIN , 3 , i16 :: MIN , 4) ; let expected = _mm_setr_epi16 (i16 :: MAX , i16 :: MAX , i16 :: MAX , i16 :: MAX , i16 :: MIN , i16 :: MIN , i16 :: MIN , i16 :: MIN ,) ; let r = _mm_hsubs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_hsub_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hsub_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hsub_epi32_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_hsub_epi32 () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let b = _mm_setr_epi32 (4 , 128 , 4 , 3) ; let expected = _mm_setr_epi32 (- 1 , - 1 , - 124 , 1) ; let r = _mm_hsub_epi32 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi32 (i32 :: MAX , - 1 , i32 :: MAX , - 2) ; let b = _mm_setr_epi32 (i32 :: MIN , 1 , i32 :: MIN , 2) ; let expected = _mm_setr_epi32 (i32 :: MIN , i32 :: MIN + 1 , i32 :: MAX , i32 :: MAX - 1) ; let r = _mm_hsub_epi32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_maddubs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maddubs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maddubs_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_maddubs_epi16 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (4 , 63 , 4 , 3 , 24 , 12 , 6 , 19 , 12 , 5 , 5 , 10 , 4 , 1 , 8 , 0 ,) ; let expected = _mm_setr_epi16 (130 , 24 , 192 , 194 , 158 , 175 , 66 , 120) ; let r = _mm_maddubs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; # [rustfmt :: skip] let a = _mm_setr_epi8 (u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , 100 , 100 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MIN , i8 :: MIN , i8 :: MIN , 50 , 15 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,) ; let expected = _mm_setr_epi16 (i16 :: MAX , - 255 , i16 :: MIN , 6500 , 0 , 0 , 0 , 0) ; let r = _mm_maddubs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_mulhrs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mulhrs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mulhrs_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_mulhrs_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 4 , 3 , 32767 , - 1 , - 32768 , 1) ; let expected = _mm_setr_epi16 (0 , 0 , 0 , 0 , 5 , 0 , - 7 , 0) ; let r = _mm_mulhrs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; let a = _mm_setr_epi16 (i16 :: MAX , i16 :: MIN , i16 :: MIN , 0 , 0 , 0 , 0 , 0) ; let b = _mm_setr_epi16 (i16 :: MAX , i16 :: MIN , i16 :: MAX , 0 , 0 , 0 , 0 , 0) ; let expected = _mm_setr_epi16 (i16 :: MAX - 1 , i16 :: MIN , - i16 :: MAX , 0 , 0 , 0 , 0 , 0) ; let r = _mm_mulhrs_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sign_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sign_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sign_epi8_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_sign_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , - 14 , - 15 , 16 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (4 , 63 , - 4 , 3 , 24 , 12 , - 6 , - 19 , 12 , 5 , - 5 , 10 , 4 , 1 , - 8 , 0 ,) ; # [rustfmt :: skip] let expected = _mm_setr_epi8 (1 , 2 , - 3 , 4 , 5 , 6 , - 7 , - 8 , 9 , 10 , - 11 , 12 , 13 , - 14 , 15 , 0 ,) ; let r = _mm_sign_epi8 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sign_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sign_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sign_epi16_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_sign_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , - 5 , - 6 , 7 , 8) ; let b = _mm_setr_epi16 (4 , 128 , 0 , 3 , 1 , - 1 , - 2 , 1) ; let expected = _mm_setr_epi16 (1 , 2 , 0 , 4 , - 5 , 6 , - 7 , 8) ; let r = _mm_sign_epi16 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sign_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sign_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sign_epi32_introspect!();
    # [simd_test (enable = "ssse3")] unsafe fn test_mm_sign_epi32 () { let a = _mm_setr_epi32 (- 1 , 2 , 3 , 4) ; let b = _mm_setr_epi32 (1 , - 1 , 1 , 0) ; let expected = _mm_setr_epi32 (- 1 , - 2 , 3 , 0) ; let r = _mm_sign_epi32 (a , b) ; assert_eq_m128i (r , expected) ; }
} 
            }}