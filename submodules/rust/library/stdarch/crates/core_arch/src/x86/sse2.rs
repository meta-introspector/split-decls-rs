mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , intrinsics :: simd :: * , intrinsics :: sqrtf64 , mem , ptr , } ;}

macro_rules! _mm_pause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_pause in module {}", module_path!());
    };
}

mkfn!{
    _mm_pause_introspect!();
    # [doc = " Provides a hint to the processor that the code sequence is a spin-wait loop."] # [doc = ""] # [doc = " This can help improve the performance and power consumption of spin-wait"] # [doc = " loops."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_pause)"] # [inline] # [cfg_attr (all (test , target_feature = "sse2") , assert_instr (pause))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_pause () { pause () }
}

macro_rules! _mm_clflush_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_clflush in module {}", module_path!());
    };
}

mkfn!{
    _mm_clflush_introspect!();
    # [doc = " Invalidates and flushes the cache line that contains `p` from all levels of"] # [doc = " the cache hierarchy."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_clflush)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (clflush))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_clflush (p : * const u8) { clflush (p) }
}

macro_rules! _mm_lfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_lfence in module {}", module_path!());
    };
}

mkfn!{
    _mm_lfence_introspect!();
    # [doc = " Performs a serializing operation on all load-from-memory instructions"] # [doc = " that were issued prior to this instruction."] # [doc = ""] # [doc = " Guarantees that every load instruction that precedes, in program order, is"] # [doc = " globally visible before any load instruction which follows the fence in"] # [doc = " program order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_lfence)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (lfence))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_lfence () { lfence () }
}

macro_rules! _mm_mfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mfence in module {}", module_path!());
    };
}

mkfn!{
    _mm_mfence_introspect!();
    # [doc = " Performs a serializing operation on all load-from-memory and store-to-memory"] # [doc = " instructions that were issued prior to this instruction."] # [doc = ""] # [doc = " Guarantees that every memory access that precedes, in program order, the"] # [doc = " memory fence instruction is globally visible before any memory instruction"] # [doc = " which follows the fence in program order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mfence)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (mfence))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_mfence () { mfence () }
}

macro_rules! _mm_add_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_epi8_introspect!();
    # [doc = " Adds packed 8-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_add (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_add_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_epi16_introspect!();
    # [doc = " Adds packed 16-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_add (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_add_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_epi32_introspect!();
    # [doc = " Adds packed 32-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_add (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_epi64_introspect!();
    # [doc = " Adds packed 64-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_epi64 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_add (a . as_i64x2 () , b . as_i64x2 ())) } }
}

macro_rules! _mm_adds_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_adds_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_adds_epi8_introspect!();
    # [doc = " Adds packed 8-bit integers in `a` and `b` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddsb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_adds_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_add (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_adds_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_adds_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_adds_epi16_introspect!();
    # [doc = " Adds packed 16-bit integers in `a` and `b` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_adds_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_add (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_adds_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_adds_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_adds_epu8_introspect!();
    # [doc = " Adds packed unsigned 8-bit integers in `a` and `b` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddusb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_adds_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_add (a . as_u8x16 () , b . as_u8x16 ())) } }
}

macro_rules! _mm_adds_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_adds_epu16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_adds_epu16_introspect!();
    # [doc = " Adds packed unsigned 16-bit integers in `a` and `b` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_adds_epu16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (paddusw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_adds_epu16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_add (a . as_u16x8 () , b . as_u16x8 ())) } }
}

macro_rules! _mm_avg_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_avg_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_avg_epu8_introspect!();
    # [doc = " Averages packed unsigned 8-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_avg_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pavgb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_avg_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = simd_cast :: < _ , u16x16 > (a . as_u8x16 ()) ; let b = simd_cast :: < _ , u16x16 > (b . as_u8x16 ()) ; let r = simd_shr (simd_add (simd_add (a , b) , u16x16 :: splat (1)) , u16x16 :: splat (1)) ; transmute (simd_cast :: < _ , u8x16 > (r)) } }
}

macro_rules! _mm_avg_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_avg_epu16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_avg_epu16_introspect!();
    # [doc = " Averages packed unsigned 16-bit integers in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_avg_epu16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pavgw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_avg_epu16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = simd_cast :: < _ , u32x8 > (a . as_u16x8 ()) ; let b = simd_cast :: < _ , u32x8 > (b . as_u16x8 ()) ; let r = simd_shr (simd_add (simd_add (a , b) , u32x8 :: splat (1)) , u32x8 :: splat (1)) ; transmute (simd_cast :: < _ , u16x8 > (r)) } }
}

macro_rules! _mm_madd_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_madd_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_madd_epi16_introspect!();
    # [doc = " Multiplies and then horizontally add signed 16 bit integers in `a` and `b`."] # [doc = ""] # [doc = " Multiplies packed signed 16-bit integers in `a` and `b`, producing"] # [doc = " intermediate signed 32-bit integers. Horizontally add adjacent pairs of"] # [doc = " intermediate 32-bit integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_madd_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmaddwd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_madd_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (pmaddwd (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_max_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_epi16_introspect!();
    # [doc = " Compares packed 16-bit integers in `a` and `b`, and returns the packed"] # [doc = " maximum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmaxsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = a . as_i16x8 () ; let b = b . as_i16x8 () ; transmute (simd_select :: < i16x8 , _ > (simd_gt (a , b) , a , b)) } }
}

macro_rules! _mm_max_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_epu8_introspect!();
    # [doc = " Compares packed unsigned 8-bit integers in `a` and `b`, and returns the"] # [doc = " packed maximum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmaxub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = a . as_u8x16 () ; let b = b . as_u8x16 () ; transmute (simd_select :: < i8x16 , _ > (simd_gt (a , b) , a , b)) } }
}

macro_rules! _mm_min_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_epi16_introspect!();
    # [doc = " Compares packed 16-bit integers in `a` and `b`, and returns the packed"] # [doc = " minimum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pminsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = a . as_i16x8 () ; let b = b . as_i16x8 () ; transmute (simd_select :: < i16x8 , _ > (simd_lt (a , b) , a , b)) } }
}

macro_rules! _mm_min_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_epu8_introspect!();
    # [doc = " Compares packed unsigned 8-bit integers in `a` and `b`, and returns the"] # [doc = " packed minimum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pminub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = a . as_u8x16 () ; let b = b . as_u8x16 () ; transmute (simd_select :: < i8x16 , _ > (simd_lt (a , b) , a , b)) } }
}

macro_rules! _mm_mulhi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mulhi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mulhi_epi16_introspect!();
    # [doc = " Multiplies the packed 16-bit integers in `a` and `b`."] # [doc = ""] # [doc = " The multiplication produces intermediate 32-bit integers, and returns the"] # [doc = " high 16 bits of the intermediate integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mulhi_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmulhw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mulhi_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = simd_cast :: < _ , i32x8 > (a . as_i16x8 ()) ; let b = simd_cast :: < _ , i32x8 > (b . as_i16x8 ()) ; let r = simd_shr (simd_mul (a , b) , i32x8 :: splat (16)) ; transmute (simd_cast :: < i32x8 , i16x8 > (r)) } }
}

macro_rules! _mm_mulhi_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mulhi_epu16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mulhi_epu16_introspect!();
    # [doc = " Multiplies the packed unsigned 16-bit integers in `a` and `b`."] # [doc = ""] # [doc = " The multiplication produces intermediate 32-bit integers, and returns the"] # [doc = " high 16 bits of the intermediate integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mulhi_epu16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmulhuw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mulhi_epu16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = simd_cast :: < _ , u32x8 > (a . as_u16x8 ()) ; let b = simd_cast :: < _ , u32x8 > (b . as_u16x8 ()) ; let r = simd_shr (simd_mul (a , b) , u32x8 :: splat (16)) ; transmute (simd_cast :: < u32x8 , u16x8 > (r)) } }
}

macro_rules! _mm_mullo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mullo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mullo_epi16_introspect!();
    # [doc = " Multiplies the packed 16-bit integers in `a` and `b`."] # [doc = ""] # [doc = " The multiplication produces intermediate 32-bit integers, and returns the"] # [doc = " low 16 bits of the intermediate integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mullo_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmullw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mullo_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_mul (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mul_epu32_introspect!();
    # [doc = " Multiplies the low unsigned 32-bit integers from each packed 64-bit element"] # [doc = " in `a` and `b`."] # [doc = ""] # [doc = " Returns the unsigned 64-bit results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mul_epu32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmuludq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mul_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { let a = a . as_u64x2 () ; let b = b . as_u64x2 () ; let mask = u64x2 :: splat (u32 :: MAX . into ()) ; transmute (simd_mul (simd_and (a , mask) , simd_and (b , mask))) } }
}

macro_rules! _mm_sad_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sad_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sad_epu8_introspect!();
    # [doc = " Sum the absolute differences of packed unsigned 8-bit integers."] # [doc = ""] # [doc = " Computes the absolute differences of packed unsigned 8-bit integers in `a`"] # [doc = " and `b`, then horizontally sum each consecutive 8 differences to produce"] # [doc = " two unsigned 16-bit integers, and pack these unsigned 16-bit integers in"] # [doc = " the low 16 bits of 64-bit elements returned."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sad_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psadbw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sad_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (psadbw (a . as_u8x16 () , b . as_u8x16 ())) } }
}

macro_rules! _mm_sub_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_epi8_introspect!();
    # [doc = " Subtracts packed 8-bit integers in `b` from packed 8-bit integers in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_sub (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_sub_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_epi16_introspect!();
    # [doc = " Subtracts packed 16-bit integers in `b` from packed 16-bit integers in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_sub (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_sub_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_epi32_introspect!();
    # [doc = " Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_sub (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_epi64_introspect!();
    # [doc = " Subtract packed 64-bit integers in `b` from packed 64-bit integers in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_epi64 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_sub (a . as_i64x2 () , b . as_i64x2 ())) } }
}

macro_rules! _mm_subs_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_subs_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_subs_epi8_introspect!();
    # [doc = " Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`"] # [doc = " using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubsb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_subs_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_sub (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_subs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_subs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_subs_epi16_introspect!();
    # [doc = " Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`"] # [doc = " using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubsw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_subs_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_sub (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_subs_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_subs_epu8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_subs_epu8_introspect!();
    # [doc = " Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit"] # [doc = " integers in `a` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_epu8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubusb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_subs_epu8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_sub (a . as_u8x16 () , b . as_u8x16 ())) } }
}

macro_rules! _mm_subs_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_subs_epu16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_subs_epu16_introspect!();
    # [doc = " Subtract packed unsigned 16-bit integers in `b` from packed unsigned 16-bit"] # [doc = " integers in `a` using saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_subs_epu16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psubusw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_subs_epu16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_saturating_sub (a . as_u16x8 () , b . as_u16x8 ())) } }
}

macro_rules! _mm_slli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_slli_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_slli_si128_introspect!();
    # [doc = " Shifts `a` left by `IMM8` bytes while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pslldq , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_slli_si128 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { _mm_slli_si128_impl :: < IMM8 > (a) } }
}

macro_rules! _mm_slli_si128_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_slli_si128_impl in module {}", module_path!());
    };
}

mkfn!{
    _mm_slli_si128_impl_introspect!();
    # [doc = " Implementation detail: converts the immediate argument of the"] # [doc = " `_mm_slli_si128` intrinsic into a compile-time constant."] # [inline] # [target_feature (enable = "sse2")] unsafe fn _mm_slli_si128_impl < const IMM8 : i32 > (a : __m128i) -> __m128i { const fn mask (shift : i32 , i : u32) -> u32 { let shift = shift as u32 & 0xff ; if shift > 15 { i } else { 16 - shift + i } } transmute :: < i8x16 , _ > (simd_shuffle ! (i8x16 :: ZERO , a . as_i8x16 () , [mask (IMM8 , 0) , mask (IMM8 , 1) , mask (IMM8 , 2) , mask (IMM8 , 3) , mask (IMM8 , 4) , mask (IMM8 , 5) , mask (IMM8 , 6) , mask (IMM8 , 7) , mask (IMM8 , 8) , mask (IMM8 , 9) , mask (IMM8 , 10) , mask (IMM8 , 11) , mask (IMM8 , 12) , mask (IMM8 , 13) , mask (IMM8 , 14) , mask (IMM8 , 15) ,] ,)) }
}

macro_rules! _mm_bslli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_bslli_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_bslli_si128_introspect!();
    # [doc = " Shifts `a` left by `IMM8` bytes while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_bslli_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pslldq , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_bslli_si128 < const IMM8 : i32 > (a : __m128i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_slli_si128_impl :: < IMM8 > (a) } }
}

macro_rules! _mm_bsrli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_bsrli_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_bsrli_si128_introspect!();
    # [doc = " Shifts `a` right by `IMM8` bytes while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_bsrli_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrldq , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_bsrli_si128 < const IMM8 : i32 > (a : __m128i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_srli_si128_impl :: < IMM8 > (a) } }
}

macro_rules! _mm_slli_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_slli_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_slli_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` left by `IMM8` while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psllw , IMM8 = 7))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_slli_epi16 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 16 { _mm_setzero_si128 () } else { transmute (simd_shl (a . as_u16x8 () , u16x8 :: splat (IMM8 as u16))) } } }
}

macro_rules! _mm_sll_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sll_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sll_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` left by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psllw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sll_epi16 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psllw (a . as_i16x8 () , count . as_i16x8 ())) } }
}

macro_rules! _mm_slli_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_slli_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_slli_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` left by `IMM8` while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pslld , IMM8 = 7))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_slli_epi32 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 32 { _mm_setzero_si128 () } else { transmute (simd_shl (a . as_u32x4 () , u32x4 :: splat (IMM8 as u32))) } } }
}

macro_rules! _mm_sll_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sll_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sll_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` left by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pslld))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sll_epi32 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (pslld (a . as_i32x4 () , count . as_i32x4 ())) } }
}

macro_rules! _mm_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_slli_epi64_introspect!();
    # [doc = " Shifts packed 64-bit integers in `a` left by `IMM8` while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_slli_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psllq , IMM8 = 7))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_slli_epi64 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 64 { _mm_setzero_si128 () } else { transmute (simd_shl (a . as_u64x2 () , u64x2 :: splat (IMM8 as u64))) } } }
}

macro_rules! _mm_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sll_epi64_introspect!();
    # [doc = " Shifts packed 64-bit integers in `a` left by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sll_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psllq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sll_epi64 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psllq (a . as_i64x2 () , count . as_i64x2 ())) } }
}

macro_rules! _mm_srai_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srai_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srai_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` right by `IMM8` while shifting in sign"] # [doc = " bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srai_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psraw , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srai_epi16 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (simd_shr (a . as_i16x8 () , i16x8 :: splat (IMM8 . min (15) as i16))) } }
}

macro_rules! _mm_sra_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sra_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sra_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` right by `count` while shifting in sign"] # [doc = " bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sra_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psraw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sra_epi16 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psraw (a . as_i16x8 () , count . as_i16x8 ())) } }
}

macro_rules! _mm_srai_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srai_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srai_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` right by `IMM8` while shifting in sign"] # [doc = " bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srai_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrad , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srai_epi32 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { transmute (simd_shr (a . as_i32x4 () , i32x4 :: splat (IMM8 . min (31)))) } }
}

macro_rules! _mm_sra_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sra_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sra_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` right by `count` while shifting in sign"] # [doc = " bits."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sra_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrad))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sra_epi32 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psrad (a . as_i32x4 () , count . as_i32x4 ())) } }
}

macro_rules! _mm_srli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srli_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srli_si128_introspect!();
    # [doc = " Shifts `a` right by `IMM8` bytes while shifting in zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrldq , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srli_si128 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { _mm_srli_si128_impl :: < IMM8 > (a) } }
}

macro_rules! _mm_srli_si128_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srli_si128_impl in module {}", module_path!());
    };
}

mkfn!{
    _mm_srli_si128_impl_introspect!();
    # [doc = " Implementation detail: converts the immediate argument of the"] # [doc = " `_mm_srli_si128` intrinsic into a compile-time constant."] # [inline] # [target_feature (enable = "sse2")] unsafe fn _mm_srli_si128_impl < const IMM8 : i32 > (a : __m128i) -> __m128i { const fn mask (shift : i32 , i : u32) -> u32 { if (shift as u32) > 15 { i + 16 } else { i + (shift as u32) } } let x : i8x16 = simd_shuffle ! (a . as_i8x16 () , i8x16 :: ZERO , [mask (IMM8 , 0) , mask (IMM8 , 1) , mask (IMM8 , 2) , mask (IMM8 , 3) , mask (IMM8 , 4) , mask (IMM8 , 5) , mask (IMM8 , 6) , mask (IMM8 , 7) , mask (IMM8 , 8) , mask (IMM8 , 9) , mask (IMM8 , 10) , mask (IMM8 , 11) , mask (IMM8 , 12) , mask (IMM8 , 13) , mask (IMM8 , 14) , mask (IMM8 , 15) ,] ,) ; transmute (x) }
}

macro_rules! _mm_srli_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srli_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srli_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` right by `IMM8` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrlw , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srli_epi16 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 16 { _mm_setzero_si128 () } else { transmute (simd_shr (a . as_u16x8 () , u16x8 :: splat (IMM8 as u16))) } } }
}

macro_rules! _mm_srl_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srl_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srl_epi16_introspect!();
    # [doc = " Shifts packed 16-bit integers in `a` right by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrlw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srl_epi16 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psrlw (a . as_i16x8 () , count . as_i16x8 ())) } }
}

macro_rules! _mm_srli_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srli_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srli_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` right by `IMM8` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrld , IMM8 = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srli_epi32 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 32 { _mm_setzero_si128 () } else { transmute (simd_shr (a . as_u32x4 () , u32x4 :: splat (IMM8 as u32))) } } }
}

macro_rules! _mm_srl_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srl_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srl_epi32_introspect!();
    # [doc = " Shifts packed 32-bit integers in `a` right by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrld))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srl_epi32 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psrld (a . as_i32x4 () , count . as_i32x4 ())) } }
}

macro_rules! _mm_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srli_epi64_introspect!();
    # [doc = " Shifts packed 64-bit integers in `a` right by `IMM8` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srli_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrlq , IMM8 = 1))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srli_epi64 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { if IMM8 >= 64 { _mm_setzero_si128 () } else { transmute (simd_shr (a . as_u64x2 () , u64x2 :: splat (IMM8 as u64))) } } }
}

macro_rules! _mm_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_srl_epi64_introspect!();
    # [doc = " Shifts packed 64-bit integers in `a` right by `count` while shifting in"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_srl_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (psrlq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_srl_epi64 (a : __m128i , count : __m128i) -> __m128i { unsafe { transmute (psrlq (a . as_i64x2 () , count . as_i64x2 ())) } }
}

macro_rules! _mm_and_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_and_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_and_si128_introspect!();
    # [doc = " Computes the bitwise AND of 128 bits (representing integer data) in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_and_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (andps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_and_si128 (a : __m128i , b : __m128i) -> __m128i { unsafe { simd_and (a , b) } }
}

macro_rules! _mm_andnot_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_andnot_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_andnot_si128_introspect!();
    # [doc = " Computes the bitwise NOT of 128 bits (representing integer data) in `a` and"] # [doc = " then AND with `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_andnot_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (andnps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_andnot_si128 (a : __m128i , b : __m128i) -> __m128i { unsafe { simd_and (simd_xor (_mm_set1_epi8 (- 1) , a) , b) } }
}

macro_rules! _mm_or_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_or_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_or_si128_introspect!();
    # [doc = " Computes the bitwise OR of 128 bits (representing integer data) in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_or_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (orps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_or_si128 (a : __m128i , b : __m128i) -> __m128i { unsafe { simd_or (a , b) } }
}

macro_rules! _mm_xor_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_xor_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_xor_si128_introspect!();
    # [doc = " Computes the bitwise XOR of 128 bits (representing integer data) in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_xor_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (xorps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_xor_si128 (a : __m128i , b : __m128i) -> __m128i { unsafe { simd_xor (a , b) } }
}

macro_rules! _mm_cmpeq_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_epi8_introspect!();
    # [doc = " Compares packed 8-bit integers in `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpeqb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i8x16 , _ > (simd_eq (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_cmpeq_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_epi16_introspect!();
    # [doc = " Compares packed 16-bit integers in `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpeqw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i16x8 , _ > (simd_eq (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_cmpeq_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_epi32_introspect!();
    # [doc = " Compares packed 32-bit integers in `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpeqd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i32x4 , _ > (simd_eq (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_cmpgt_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_epi8_introspect!();
    # [doc = " Compares packed 8-bit integers in `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i8x16 , _ > (simd_gt (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_cmpgt_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_epi16_introspect!();
    # [doc = " Compares packed 16-bit integers in `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i16x8 , _ > (simd_gt (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_cmpgt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_epi32_introspect!();
    # [doc = " Compares packed 32-bit integers in `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i32x4 , _ > (simd_gt (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_cmplt_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_epi8_introspect!();
    # [doc = " Compares packed 8-bit integers in `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i8x16 , _ > (simd_lt (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_cmplt_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_epi16_introspect!();
    # [doc = " Compares packed 16-bit integers in `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i16x8 , _ > (simd_lt (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_cmplt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_epi32_introspect!();
    # [doc = " Compares packed 32-bit integers in `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pcmpgtd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i32x4 , _ > (simd_lt (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepi32_pd_introspect!();
    # [doc = " Converts the lower two packed 32-bit integers in `a` to packed"] # [doc = " double-precision (64-bit) floating-point elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepi32_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtdq2pd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtepi32_pd (a : __m128i) -> __m128d { unsafe { let a = a . as_i32x4 () ; simd_cast :: < i32x2 , __m128d > (simd_shuffle ! (a , a , [0 , 1])) } }
}

macro_rules! _mm_cvtsi32_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi32_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi32_sd_introspect!();
    # [doc = " Returns `a` with its lower element replaced by `b` after converting it to"] # [doc = " an `f64`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi32_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsi2sd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi32_sd (a : __m128d , b : i32) -> __m128d { unsafe { simd_insert ! (a , 0 , b as f64) } }
}

macro_rules! _mm_cvtepi32_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepi32_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepi32_ps_introspect!();
    # [doc = " Converts packed 32-bit integers in `a` to packed single-precision (32-bit)"] # [doc = " floating-point elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepi32_ps)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtdq2ps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtepi32_ps (a : __m128i) -> __m128 { unsafe { transmute (simd_cast :: < _ , f32x4 > (a . as_i32x4 ())) } }
}

macro_rules! _mm_cvtps_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtps_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtps_epi32_introspect!();
    # [doc = " Converts packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " to packed 32-bit integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtps_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtps2dq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtps_epi32 (a : __m128) -> __m128i { unsafe { transmute (cvtps2dq (a)) } }
}

macro_rules! _mm_cvtsi32_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi32_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi32_si128_introspect!();
    # [doc = " Returns a vector whose lowest element is `a` and all higher elements are"] # [doc = " `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi32_si128)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi32_si128 (a : i32) -> __m128i { unsafe { transmute (i32x4 :: new (a , 0 , 0 , 0)) } }
}

macro_rules! _mm_cvtsi128_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi128_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi128_si32_introspect!();
    # [doc = " Returns the lowest element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi128_si32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi128_si32 (a : __m128i) -> i32 { unsafe { simd_extract ! (a . as_i32x4 () , 0) } }
}

macro_rules! _mm_set_epi64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_epi64x in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_epi64x_introspect!();
    # [doc = " Sets packed 64-bit integers with the supplied values, from highest to"] # [doc = " lowest."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_epi64x)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_epi64x (e1 : i64 , e0 : i64) -> __m128i { unsafe { transmute (i64x2 :: new (e0 , e1)) } }
}

macro_rules! _mm_set_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_epi32_introspect!();
    # [doc = " Sets packed 32-bit integers with the supplied values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_epi32 (e3 : i32 , e2 : i32 , e1 : i32 , e0 : i32) -> __m128i { unsafe { transmute (i32x4 :: new (e0 , e1 , e2 , e3)) } }
}

macro_rules! _mm_set_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_epi16_introspect!();
    # [doc = " Sets packed 16-bit integers with the supplied values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_epi16 (e7 : i16 , e6 : i16 , e5 : i16 , e4 : i16 , e3 : i16 , e2 : i16 , e1 : i16 , e0 : i16 ,) -> __m128i { unsafe { transmute (i16x8 :: new (e0 , e1 , e2 , e3 , e4 , e5 , e6 , e7)) } }
}

macro_rules! _mm_set_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_epi8_introspect!();
    # [doc = " Sets packed 8-bit integers with the supplied values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_epi8 (e15 : i8 , e14 : i8 , e13 : i8 , e12 : i8 , e11 : i8 , e10 : i8 , e9 : i8 , e8 : i8 , e7 : i8 , e6 : i8 , e5 : i8 , e4 : i8 , e3 : i8 , e2 : i8 , e1 : i8 , e0 : i8 ,) -> __m128i { unsafe { # [rustfmt :: skip] transmute (i8x16 :: new (e0 , e1 , e2 , e3 , e4 , e5 , e6 , e7 , e8 , e9 , e10 , e11 , e12 , e13 , e14 , e15 ,)) } }
}

macro_rules! _mm_set1_epi64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_epi64x in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_epi64x_introspect!();
    # [doc = " Broadcasts 64-bit integer `a` to all elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_epi64x)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_epi64x (a : i64) -> __m128i { _mm_set_epi64x (a , a) }
}

macro_rules! _mm_set1_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_epi32_introspect!();
    # [doc = " Broadcasts 32-bit integer `a` to all elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_epi32 (a : i32) -> __m128i { _mm_set_epi32 (a , a , a , a) }
}

macro_rules! _mm_set1_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_epi16_introspect!();
    # [doc = " Broadcasts 16-bit integer `a` to all elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_epi16 (a : i16) -> __m128i { _mm_set_epi16 (a , a , a , a , a , a , a , a) }
}

macro_rules! _mm_set1_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_epi8_introspect!();
    # [doc = " Broadcasts 8-bit integer `a` to all elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_epi8 (a : i8) -> __m128i { _mm_set_epi8 (a , a , a , a , a , a , a , a , a , a , a , a , a , a , a , a) }
}

macro_rules! _mm_setr_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setr_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_setr_epi32_introspect!();
    # [doc = " Sets packed 32-bit integers with the supplied values in reverse order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setr_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setr_epi32 (e3 : i32 , e2 : i32 , e1 : i32 , e0 : i32) -> __m128i { _mm_set_epi32 (e0 , e1 , e2 , e3) }
}

macro_rules! _mm_setr_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setr_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_setr_epi16_introspect!();
    # [doc = " Sets packed 16-bit integers with the supplied values in reverse order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setr_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setr_epi16 (e7 : i16 , e6 : i16 , e5 : i16 , e4 : i16 , e3 : i16 , e2 : i16 , e1 : i16 , e0 : i16 ,) -> __m128i { _mm_set_epi16 (e0 , e1 , e2 , e3 , e4 , e5 , e6 , e7) }
}

macro_rules! _mm_setr_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setr_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_setr_epi8_introspect!();
    # [doc = " Sets packed 8-bit integers with the supplied values in reverse order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setr_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setr_epi8 (e15 : i8 , e14 : i8 , e13 : i8 , e12 : i8 , e11 : i8 , e10 : i8 , e9 : i8 , e8 : i8 , e7 : i8 , e6 : i8 , e5 : i8 , e4 : i8 , e3 : i8 , e2 : i8 , e1 : i8 , e0 : i8 ,) -> __m128i { # [rustfmt :: skip] _mm_set_epi8 (e0 , e1 , e2 , e3 , e4 , e5 , e6 , e7 , e8 , e9 , e10 , e11 , e12 , e13 , e14 , e15 ,) }
}

macro_rules! _mm_setzero_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setzero_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_setzero_si128_introspect!();
    # [doc = " Returns a vector with all elements set to zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setzero_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (xorps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setzero_si128 () -> __m128i { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _mm_loadl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadl_epi64_introspect!();
    # [doc = " Loads 64-bit integer from memory into first element of returned vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadl_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadl_epi64 (mem_addr : * const __m128i) -> __m128i { _mm_set_epi64x (0 , ptr :: read_unaligned (mem_addr as * const i64)) }
}

macro_rules! _mm_load_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_si128_introspect!();
    # [doc = " Loads 128-bits of integer data from memory into a new vector."] # [doc = ""] # [doc = " `mem_addr` must be aligned on a 16-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load_si128 (mem_addr : * const __m128i) -> __m128i { * mem_addr }
}

macro_rules! _mm_loadu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_si128_introspect!();
    # [doc = " Loads 128-bits of integer data from memory into a new vector."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadu_si128 (mem_addr : * const __m128i) -> __m128i { let mut dst : __m128i = _mm_undefined_si128 () ; ptr :: copy_nonoverlapping (mem_addr as * const u8 , ptr :: addr_of_mut ! (dst) as * mut u8 , mem :: size_of :: < __m128i > () ,) ; dst }
}

macro_rules! _mm_maskmoveu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskmoveu_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskmoveu_si128_introspect!();
    # [doc = " Conditionally store 8-bit integer elements from `a` into memory using"] # [doc = " `mask` flagged as non-temporal (unlikely to be used again soon)."] # [doc = ""] # [doc = " Elements are not stored when the highest bit is not set in the"] # [doc = " corresponding element."] # [doc = ""] # [doc = " `mem_addr` should correspond to a 128-bit memory location and does not need"] # [doc = " to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskmoveu_si128)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (maskmovdqu))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_maskmoveu_si128 (a : __m128i , mask : __m128i , mem_addr : * mut i8) { maskmovdqu (a . as_i8x16 () , mask . as_i8x16 () , mem_addr) }
}

macro_rules! _mm_store_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_si128_introspect!();
    # [doc = " Stores 128-bits of integer data from `a` into memory."] # [doc = ""] # [doc = " `mem_addr` must be aligned on a 16-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_store_si128 (mem_addr : * mut __m128i , a : __m128i) { * mem_addr = a ; }
}

macro_rules! _mm_storeu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_si128_introspect!();
    # [doc = " Stores 128-bits of integer data from `a` into memory."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_si128)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storeu_si128 (mem_addr : * mut __m128i , a : __m128i) { mem_addr . write_unaligned (a) ; }
}

macro_rules! _mm_storel_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storel_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_storel_epi64_introspect!();
    # [doc = " Stores the lower 64-bit integer `a` to a memory location."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storel_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storel_epi64 (mem_addr : * mut __m128i , a : __m128i) { ptr :: copy_nonoverlapping (ptr :: addr_of ! (a) as * const u8 , mem_addr as * mut u8 , 8) ; }
}

macro_rules! _mm_stream_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_si128_introspect!();
    # [doc = " Stores a 128-bit integer vector to a 128-bit aligned memory location."] # [doc = " To minimize caching, the data is flagged as non-temporal (unlikely to be"] # [doc = " used again soon)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_stream_si128)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movntdq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_stream_si128 (mem_addr : * mut __m128i , a : __m128i) { crate :: arch :: asm ! (vps ! ("movntdq" , ",{a}") , p = in (reg) mem_addr , a = in (xmm_reg) a , options (nostack , preserves_flags) ,) ; }
}

macro_rules! _mm_stream_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_si32_introspect!();
    # [doc = " Stores a 32-bit integer value in the specified memory location."] # [doc = " To minimize caching, the data is flagged as non-temporal (unlikely to be"] # [doc = " used again soon)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_stream_si32)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movnti))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_stream_si32 (mem_addr : * mut i32 , a : i32) { crate :: arch :: asm ! (vps ! ("movnti" , ",{a:e}") , p = in (reg) mem_addr , a = in (reg) a , options (nostack , preserves_flags) ,) ; }
}

macro_rules! _mm_move_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_move_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_move_epi64_introspect!();
    # [doc = " Returns a vector where the low element is extracted from `a` and its upper"] # [doc = " element is zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_move_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , target_arch = "x86_64") , assert_instr (movq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_move_epi64 (a : __m128i) -> __m128i { unsafe { let r : i64x2 = simd_shuffle ! (a . as_i64x2 () , i64x2 :: ZERO , [0 , 2]) ; transmute (r) } }
}

macro_rules! _mm_packs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_packs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_packs_epi16_introspect!();
    # [doc = " Converts packed 16-bit integers from `a` and `b` to packed 8-bit integers"] # [doc = " using signed saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packs_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (packsswb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_packs_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (packsswb (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_packs_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_packs_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_packs_epi32_introspect!();
    # [doc = " Converts packed 32-bit integers from `a` and `b` to packed 16-bit integers"] # [doc = " using signed saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packs_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (packssdw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_packs_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (packssdw (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_packus_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_packus_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_packus_epi16_introspect!();
    # [doc = " Converts packed 16-bit integers from `a` and `b` to packed 8-bit integers"] # [doc = " using unsigned saturation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_packus_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (packuswb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_packus_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (packuswb (a . as_i16x8 () , b . as_i16x8 ())) } }
}

macro_rules! _mm_extract_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_extract_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_extract_epi16_introspect!();
    # [doc = " Returns the `imm8` element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_extract_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pextrw , IMM8 = 7))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_extract_epi16 < const IMM8 : i32 > (a : __m128i) -> i32 { static_assert_uimm_bits ! (IMM8 , 3) ; unsafe { simd_extract ! (a . as_u16x8 () , IMM8 as u32 , u16) as i32 } }
}

macro_rules! _mm_insert_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_insert_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_insert_epi16_introspect!();
    # [doc = " Returns a new vector where the `imm8` element of `a` is replaced with `i`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_insert_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pinsrw , IMM8 = 7))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_insert_epi16 < const IMM8 : i32 > (a : __m128i , i : i32) -> __m128i { static_assert_uimm_bits ! (IMM8 , 3) ; unsafe { transmute (simd_insert ! (a . as_i16x8 () , IMM8 as u32 , i as i16)) } }
}

macro_rules! _mm_movemask_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movemask_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_movemask_epi8_introspect!();
    # [doc = " Returns a mask of the most significant bit of each element in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movemask_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pmovmskb))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movemask_epi8 (a : __m128i) -> i32 { unsafe { let z = i8x16 :: ZERO ; let m : i8x16 = simd_lt (a . as_i8x16 () , z) ; simd_bitmask :: < _ , u16 > (m) as u32 as i32 } }
}

macro_rules! _mm_shuffle_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shuffle_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_shuffle_epi32_introspect!();
    # [doc = " Shuffles 32-bit integers in `a` using the control in `IMM8`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shuffle_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pshufd , IMM8 = 9))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shuffle_epi32 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { let a = a . as_i32x4 () ; let x : i32x4 = simd_shuffle ! (a , a , [IMM8 as u32 & 0b11 , (IMM8 as u32 >> 2) & 0b11 , (IMM8 as u32 >> 4) & 0b11 , (IMM8 as u32 >> 6) & 0b11 ,] ,) ; transmute (x) } }
}

macro_rules! _mm_shufflehi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shufflehi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_shufflehi_epi16_introspect!();
    # [doc = " Shuffles 16-bit integers in the high 64 bits of `a` using the control in"] # [doc = " `IMM8`."] # [doc = ""] # [doc = " Put the results in the high 64 bits of the returned vector, with the low 64"] # [doc = " bits being copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shufflehi_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pshufhw , IMM8 = 9))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shufflehi_epi16 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { let a = a . as_i16x8 () ; let x : i16x8 = simd_shuffle ! (a , a , [0 , 1 , 2 , 3 , (IMM8 as u32 & 0b11) + 4 , ((IMM8 as u32 >> 2) & 0b11) + 4 , ((IMM8 as u32 >> 4) & 0b11) + 4 , ((IMM8 as u32 >> 6) & 0b11) + 4 ,] ,) ; transmute (x) } }
}

macro_rules! _mm_shufflelo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shufflelo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_shufflelo_epi16_introspect!();
    # [doc = " Shuffles 16-bit integers in the low 64 bits of `a` using the control in"] # [doc = " `IMM8`."] # [doc = ""] # [doc = " Put the results in the low 64 bits of the returned vector, with the high 64"] # [doc = " bits being copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shufflelo_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (pshuflw , IMM8 = 9))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shufflelo_epi16 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { let a = a . as_i16x8 () ; let x : i16x8 = simd_shuffle ! (a , a , [IMM8 as u32 & 0b11 , (IMM8 as u32 >> 2) & 0b11 , (IMM8 as u32 >> 4) & 0b11 , (IMM8 as u32 >> 6) & 0b11 , 4 , 5 , 6 , 7 ,] ,) ; transmute (x) } }
}

macro_rules! _mm_unpackhi_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_epi8_introspect!();
    # [doc = " Unpacks and interleave 8-bit integers from the high half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (punpckhbw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i8x16 , _ > (simd_shuffle ! (a . as_i8x16 () , b . as_i8x16 () , [8 , 24 , 9 , 25 , 10 , 26 , 11 , 27 , 12 , 28 , 13 , 29 , 14 , 30 , 15 , 31] ,)) } }
}

macro_rules! _mm_unpackhi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_epi16_introspect!();
    # [doc = " Unpacks and interleave 16-bit integers from the high half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (punpckhwd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let x = simd_shuffle ! (a . as_i16x8 () , b . as_i16x8 () , [4 , 12 , 5 , 13 , 6 , 14 , 7 , 15]) ; transmute :: < i16x8 , _ > (x) } }
}

macro_rules! _mm_unpackhi_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_epi32_introspect!();
    # [doc = " Unpacks and interleave 32-bit integers from the high half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (unpckhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i32x4 , _ > (simd_shuffle ! (a . as_i32x4 () , b . as_i32x4 () , [2 , 6 , 3 , 7])) } }
}

macro_rules! _mm_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_epi64_introspect!();
    # [doc = " Unpacks and interleave 64-bit integers from the high half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (unpckhpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_epi64 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i64x2 , _ > (simd_shuffle ! (a . as_i64x2 () , b . as_i64x2 () , [1 , 3])) } }
}

macro_rules! _mm_unpacklo_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_epi8_introspect!();
    # [doc = " Unpacks and interleave 8-bit integers from the low half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_epi8)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (punpcklbw))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i8x16 , _ > (simd_shuffle ! (a . as_i8x16 () , b . as_i8x16 () , [0 , 16 , 1 , 17 , 2 , 18 , 3 , 19 , 4 , 20 , 5 , 21 , 6 , 22 , 7 , 23] ,)) } }
}

macro_rules! _mm_unpacklo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_epi16_introspect!();
    # [doc = " Unpacks and interleave 16-bit integers from the low half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_epi16)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (punpcklwd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_epi16 (a : __m128i , b : __m128i) -> __m128i { unsafe { let x = simd_shuffle ! (a . as_i16x8 () , b . as_i16x8 () , [0 , 8 , 1 , 9 , 2 , 10 , 3 , 11]) ; transmute :: < i16x8 , _ > (x) } }
}

macro_rules! _mm_unpacklo_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_epi32_introspect!();
    # [doc = " Unpacks and interleave 32-bit integers from the low half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (unpcklps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i32x4 , _ > (simd_shuffle ! (a . as_i32x4 () , b . as_i32x4 () , [0 , 4 , 1 , 5])) } }
}

macro_rules! _mm_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_epi64_introspect!();
    # [doc = " Unpacks and interleave 64-bit integers from the low half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_epi64)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movlhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_epi64 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute :: < i64x2 , _ > (simd_shuffle ! (a . as_i64x2 () , b . as_i64x2 () , [0 , 2])) } }
}

macro_rules! _mm_add_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the sum of the"] # [doc = " low elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (addsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , _mm_cvtsd_f64 (a) + _mm_cvtsd_f64 (b)) } }
}

macro_rules! _mm_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_pd_introspect!();
    # [doc = " Adds packed double-precision (64-bit) floating-point elements in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (addpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_add (a , b) } }
}

macro_rules! _mm_div_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_div_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_div_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the result of"] # [doc = " diving the lower element of `a` by the lower element of `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_div_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (divsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_div_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , _mm_cvtsd_f64 (a) / _mm_cvtsd_f64 (b)) } }
}

macro_rules! _mm_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_div_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_div_pd_introspect!();
    # [doc = " Divide packed double-precision (64-bit) floating-point elements in `a` by"] # [doc = " packed elements in `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_div_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (divpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_div_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_div (a , b) } }
}

macro_rules! _mm_max_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the maximum"] # [doc = " of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (maxsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { maxsd (a , b) } }
}

macro_rules! _mm_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_pd_introspect!();
    # [doc = " Returns a new vector with the maximum values from corresponding elements in"] # [doc = " `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (maxpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { maxpd (a , b) } }
}

macro_rules! _mm_min_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the minimum"] # [doc = " of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (minsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { minsd (a , b) } }
}

macro_rules! _mm_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_pd_introspect!();
    # [doc = " Returns a new vector with the minimum values from corresponding elements in"] # [doc = " `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (minpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { minpd (a , b) } }
}

macro_rules! _mm_mul_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mul_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mul_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by multiplying the"] # [doc = " low elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mul_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (mulsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mul_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , _mm_cvtsd_f64 (a) * _mm_cvtsd_f64 (b)) } }
}

macro_rules! _mm_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mul_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mul_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (mulpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mul_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_mul (a , b) } }
}

macro_rules! _mm_sqrt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sqrt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_sqrt_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the square"] # [doc = " root of the lower element `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sqrt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (sqrtsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sqrt_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , sqrtf64 (_mm_cvtsd_f64 (b))) } }
}

macro_rules! _mm_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_sqrt_pd_introspect!();
    # [doc = " Returns a new vector with the square root of each of the values in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sqrt_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (sqrtpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sqrt_pd (a : __m128d) -> __m128d { unsafe { simd_fsqrt (a) } }
}

macro_rules! _mm_sub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by subtracting the"] # [doc = " low element by `b` from the low element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (subsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , _mm_cvtsd_f64 (a) - _mm_cvtsd_f64 (b)) } }
}

macro_rules! _mm_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_pd_introspect!();
    # [doc = " Subtract packed double-precision (64-bit) floating-point elements in `b`"] # [doc = " from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (subpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_sub (a , b) } }
}

macro_rules! _mm_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_and_pd_introspect!();
    # [doc = " Computes the bitwise AND of packed double-precision (64-bit) floating-point"] # [doc = " elements in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_and_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (andps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_and_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { let a : __m128i = transmute (a) ; let b : __m128i = transmute (b) ; transmute (_mm_and_si128 (a , b)) } }
}

macro_rules! _mm_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_andnot_pd_introspect!();
    # [doc = " Computes the bitwise NOT of `a` and then AND with `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_andnot_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (andnps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_andnot_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { let a : __m128i = transmute (a) ; let b : __m128i = transmute (b) ; transmute (_mm_andnot_si128 (a , b)) } }
}

macro_rules! _mm_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_or_pd_introspect!();
    # [doc = " Computes the bitwise OR of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_or_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (orps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_or_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { let a : __m128i = transmute (a) ; let b : __m128i = transmute (b) ; transmute (_mm_or_si128 (a , b)) } }
}

macro_rules! _mm_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_xor_pd_introspect!();
    # [doc = " Computes the bitwise XOR of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_xor_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (xorps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_xor_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { let a : __m128i = transmute (a) ; let b : __m128i = transmute (b) ; transmute (_mm_xor_si128 (a , b)) } }
}

macro_rules! _mm_cmpeq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the equality"] # [doc = " comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpeqsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 0) } }
}

macro_rules! _mm_cmplt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the less-than"] # [doc = " comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpltsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 1) } }
}

macro_rules! _mm_cmple_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmple_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmple_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " less-than-or-equal comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmple_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmplesd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmple_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 2) } }
}

macro_rules! _mm_cmpgt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " greater-than comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpltsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (_mm_cmplt_sd (b , a) , 1 , simd_extract ! (a , 1 , f64)) } }
}

macro_rules! _mm_cmpge_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpge_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpge_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " greater-than-or-equal comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpge_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmplesd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpge_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (_mm_cmple_sd (b , a) , 1 , simd_extract ! (a , 1 , f64)) } }
}

macro_rules! _mm_cmpord_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpord_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpord_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the result"] # [doc = " of comparing both of the lower elements of `a` and `b` to `NaN`. If"] # [doc = " neither are equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpord_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpordsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpord_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 7) } }
}

macro_rules! _mm_cmpunord_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpunord_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpunord_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the result of"] # [doc = " comparing both of the lower elements of `a` and `b` to `NaN`. If either is"] # [doc = " equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpunord_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpunordsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpunord_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 3) } }
}

macro_rules! _mm_cmpneq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpneq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpneq_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the not-equal"] # [doc = " comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpneq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpneqsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpneq_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 4) } }
}

macro_rules! _mm_cmpnlt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnlt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnlt_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " not-less-than comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnlt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnltsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnlt_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 5) } }
}

macro_rules! _mm_cmpnle_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnle_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnle_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " not-less-than-or-equal comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnle_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnlesd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnle_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmpsd (a , b , 6) } }
}

macro_rules! _mm_cmpngt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpngt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpngt_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " not-greater-than comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpngt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnltsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpngt_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (_mm_cmpnlt_sd (b , a) , 1 , simd_extract ! (a , 1 , f64)) } }
}

macro_rules! _mm_cmpnge_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnge_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnge_sd_introspect!();
    # [doc = " Returns a new vector with the low element of `a` replaced by the"] # [doc = " not-greater-than-or-equal comparison of the lower elements of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnge_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnlesd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnge_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_insert ! (_mm_cmpnle_sd (b , a) , 1 , simd_extract ! (a , 1 , f64)) } }
}

macro_rules! _mm_cmpeq_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpeqpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 0) } }
}

macro_rules! _mm_cmplt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpltpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 1) } }
}

macro_rules! _mm_cmple_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmple_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmple_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for less-than-or-equal"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmple_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmplepd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmple_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 2) } }
}

macro_rules! _mm_cmpgt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpltpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_pd (a : __m128d , b : __m128d) -> __m128d { _mm_cmplt_pd (b , a) }
}

macro_rules! _mm_cmpge_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpge_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpge_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for greater-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpge_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmplepd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpge_pd (a : __m128d , b : __m128d) -> __m128d { _mm_cmple_pd (b , a) }
}

macro_rules! _mm_cmpord_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpord_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpord_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` to see if neither is `NaN`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpord_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpordpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpord_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 7) } }
}

macro_rules! _mm_cmpunord_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpunord_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpunord_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` to see if either is `NaN`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpunord_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpunordpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpunord_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 3) } }
}

macro_rules! _mm_cmpneq_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpneq_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpneq_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for not-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpneq_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpneqpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpneq_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 4) } }
}

macro_rules! _mm_cmpnlt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnlt_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnlt_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for not-less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnlt_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnltpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnlt_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 5) } }
}

macro_rules! _mm_cmpnle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnle_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnle_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for not-less-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnle_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnlepd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnle_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { cmppd (a , b , 6) } }
}

macro_rules! _mm_cmpngt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpngt_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpngt_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for not-greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpngt_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnltpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpngt_pd (a : __m128d , b : __m128d) -> __m128d { _mm_cmpnlt_pd (b , a) }
}

macro_rules! _mm_cmpnge_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnge_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnge_pd_introspect!();
    # [doc = " Compares corresponding elements in `a` and `b` for"] # [doc = " not-greater-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnge_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cmpnlepd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnge_pd (a : __m128d , b : __m128d) -> __m128d { _mm_cmpnle_pd (b , a) }
}

macro_rules! _mm_comieq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comieq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comieq_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comieq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comieq_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comieqsd (a , b) } }
}

macro_rules! _mm_comilt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comilt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comilt_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comilt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comilt_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comiltsd (a , b) } }
}

macro_rules! _mm_comile_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comile_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comile_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for less-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comile_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comile_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comilesd (a , b) } }
}

macro_rules! _mm_comigt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comigt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comigt_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comigt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comigt_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comigtsd (a , b) } }
}

macro_rules! _mm_comige_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comige_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comige_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for greater-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comige_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comige_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comigesd (a , b) } }
}

macro_rules! _mm_comineq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comineq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_comineq_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for not-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comineq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (comisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comineq_sd (a : __m128d , b : __m128d) -> i32 { unsafe { comineqsd (a , b) } }
}

macro_rules! _mm_ucomieq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomieq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomieq_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for equality."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomieq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomieq_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomieqsd (a , b) } }
}

macro_rules! _mm_ucomilt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomilt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomilt_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for less-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomilt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomilt_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomiltsd (a , b) } }
}

macro_rules! _mm_ucomile_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomile_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomile_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for less-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomile_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomile_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomilesd (a , b) } }
}

macro_rules! _mm_ucomigt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomigt_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomigt_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for greater-than."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomigt_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomigt_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomigtsd (a , b) } }
}

macro_rules! _mm_ucomige_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomige_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomige_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for greater-than-or-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomige_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomige_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomigesd (a , b) } }
}

macro_rules! _mm_ucomineq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomineq_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomineq_sd_introspect!();
    # [doc = " Compares the lower element of `a` and `b` for not-equal."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomineq_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (ucomisd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomineq_sd (a : __m128d , b : __m128d) -> i32 { unsafe { ucomineqsd (a , b) } }
}

macro_rules! _mm_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtpd_ps_introspect!();
    # [doc = " Converts packed double-precision (64-bit) floating-point elements in `a` to"] # [doc = " packed single-precision (32-bit) floating-point elements"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtpd_ps)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtpd2ps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtpd_ps (a : __m128d) -> __m128 { unsafe { let r = simd_cast :: < _ , f32x2 > (a . as_f64x2 ()) ; let zero = f32x2 :: ZERO ; transmute :: < f32x4 , _ > (simd_shuffle ! (r , zero , [0 , 1 , 2 , 3])) } }
}

macro_rules! _mm_cvtps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtps_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtps_pd_introspect!();
    # [doc = " Converts packed single-precision (32-bit) floating-point elements in `a` to"] # [doc = " packed"] # [doc = " double-precision (64-bit) floating-point elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtps_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtps2pd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtps_pd (a : __m128) -> __m128d { unsafe { let a = a . as_f32x4 () ; transmute (simd_cast :: < f32x2 , f64x2 > (simd_shuffle ! (a , a , [0 , 1]))) } }
}

macro_rules! _mm_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtpd_epi32_introspect!();
    # [doc = " Converts packed double-precision (64-bit) floating-point elements in `a` to"] # [doc = " packed 32-bit integers."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtpd_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtpd2dq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtpd_epi32 (a : __m128d) -> __m128i { unsafe { transmute (cvtpd2dq (a)) } }
}

macro_rules! _mm_cvtsd_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_si32_introspect!();
    # [doc = " Converts the lower double-precision (64-bit) floating-point element in a to"] # [doc = " a 32-bit integer."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_si32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsd_si32 (a : __m128d) -> i32 { unsafe { cvtsd2si (a) } }
}

macro_rules! _mm_cvtsd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_ss_introspect!();
    # [doc = " Converts the lower double-precision (64-bit) floating-point element in `b`"] # [doc = " to a single-precision (32-bit) floating-point element, store the result in"] # [doc = " the lower element of the return value, and copies the upper element from `a`"] # [doc = " to the upper element the return value."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_ss)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtsd2ss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsd_ss (a : __m128 , b : __m128d) -> __m128 { unsafe { cvtsd2ss (a , b) } }
}

macro_rules! _mm_cvtsd_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_f64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_f64_introspect!();
    # [doc = " Returns the lower double-precision (64-bit) floating-point element of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_f64)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsd_f64 (a : __m128d) -> f64 { unsafe { simd_extract ! (a , 0) } }
}

macro_rules! _mm_cvtss_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_sd_introspect!();
    # [doc = " Converts the lower single-precision (32-bit) floating-point element in `b`"] # [doc = " to a double-precision (64-bit) floating-point element, store the result in"] # [doc = " the lower element of the return value, and copies the upper element from `a`"] # [doc = " to the upper element the return value."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvtss2sd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtss_sd (a : __m128d , b : __m128) -> __m128d { unsafe { cvtss2sd (a , b) } }
}

macro_rules! _mm_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttpd_epi32_introspect!();
    # [doc = " Converts packed double-precision (64-bit) floating-point elements in `a` to"] # [doc = " packed 32-bit integers with truncation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttpd_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvttpd2dq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttpd_epi32 (a : __m128d) -> __m128i { unsafe { transmute (cvttpd2dq (a)) } }
}

macro_rules! _mm_cvttsd_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttsd_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttsd_si32_introspect!();
    # [doc = " Converts the lower double-precision (64-bit) floating-point element in `a`"] # [doc = " to a 32-bit integer with truncation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttsd_si32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvttsd2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttsd_si32 (a : __m128d) -> i32 { unsafe { cvttsd2si (a) } }
}

macro_rules! _mm_cvttps_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttps_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttps_epi32_introspect!();
    # [doc = " Converts packed single-precision (32-bit) floating-point elements in `a` to"] # [doc = " packed 32-bit integers with truncation."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttps_epi32)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (cvttps2dq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttps_epi32 (a : __m128) -> __m128i { unsafe { transmute (cvttps2dq (a)) } }
}

macro_rules! _mm_set_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_sd_introspect!();
    # [doc = " Copies double-precision (64-bit) floating-point element `a` to the lower"] # [doc = " element of the packed 64-bit return value."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_sd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_sd (a : f64) -> __m128d { _mm_set_pd (0.0 , a) }
}

macro_rules! _mm_set1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_pd_introspect!();
    # [doc = " Broadcasts double-precision (64-bit) floating-point value a to all elements"] # [doc = " of the return value."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_pd (a : f64) -> __m128d { _mm_set_pd (a , a) }
}

macro_rules! _mm_set_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_pd1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_pd1_introspect!();
    # [doc = " Broadcasts double-precision (64-bit) floating-point value a to all elements"] # [doc = " of the return value."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_pd1)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_pd1 (a : f64) -> __m128d { _mm_set_pd (a , a) }
}

macro_rules! _mm_set_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_pd_introspect!();
    # [doc = " Sets packed double-precision (64-bit) floating-point elements in the return"] # [doc = " value with the supplied values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_pd (a : f64 , b : f64) -> __m128d { __m128d ([b , a]) }
}

macro_rules! _mm_setr_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setr_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_setr_pd_introspect!();
    # [doc = " Sets packed double-precision (64-bit) floating-point elements in the return"] # [doc = " value with the supplied values in reverse order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setr_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setr_pd (a : f64 , b : f64) -> __m128d { _mm_set_pd (b , a) }
}

macro_rules! _mm_setzero_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setzero_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_setzero_pd_introspect!();
    # [doc = " Returns packed double-precision (64-bit) floating-point elements with all"] # [doc = " zeros."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setzero_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (xorp))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setzero_pd () -> __m128d { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _mm_movemask_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movemask_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_movemask_pd_introspect!();
    # [doc = " Returns a mask of the most significant bit of each element in `a`."] # [doc = ""] # [doc = " The mask is stored in the 2 least significant bits of the return value."] # [doc = " All other bits are set to `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movemask_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movmskpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movemask_pd (a : __m128d) -> i32 { unsafe { let mask : i64x2 = simd_lt (transmute (a) , i64x2 :: ZERO) ; simd_bitmask :: < i64x2 , u8 > (mask) . into () } }
}

macro_rules! _mm_load_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_pd_introspect!();
    # [doc = " Loads 128-bits (composed of 2 packed double-precision (64-bit)"] # [doc = " floating-point elements) from memory into the returned vector."] # [doc = " `mem_addr` must be aligned on a 16-byte boundary or a general-protection"] # [doc = " exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_load_pd (mem_addr : * const f64) -> __m128d { * (mem_addr as * const __m128d) }
}

macro_rules! _mm_load_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_sd_introspect!();
    # [doc = " Loads a 64-bit double-precision value to the low element of a"] # [doc = " 128-bit integer vector and clears the upper element."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load_sd (mem_addr : * const f64) -> __m128d { _mm_setr_pd (* mem_addr , 0.) }
}

macro_rules! _mm_loadh_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadh_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadh_pd_introspect!();
    # [doc = " Loads a double-precision value into the high-order bits of a 128-bit"] # [doc = " vector of `[2 x double]`. The low-order bits are copied from the low-order"] # [doc = " bits of the first operand."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadh_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadh_pd (a : __m128d , mem_addr : * const f64) -> __m128d { _mm_setr_pd (simd_extract ! (a , 0) , * mem_addr) }
}

macro_rules! _mm_loadl_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadl_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadl_pd_introspect!();
    # [doc = " Loads a double-precision value into the low-order bits of a 128-bit"] # [doc = " vector of `[2 x double]`. The high-order bits are copied from the"] # [doc = " high-order bits of the first operand."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadl_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movlps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadl_pd (a : __m128d , mem_addr : * const f64) -> __m128d { _mm_setr_pd (* mem_addr , simd_extract ! (a , 1)) }
}

macro_rules! _mm_stream_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_pd_introspect!();
    # [doc = " Stores a 128-bit floating point vector of `[2 x double]` to a 128-bit"] # [doc = " aligned memory location."] # [doc = " To minimize caching, the data is flagged as non-temporal (unlikely to be"] # [doc = " used again soon)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_stream_pd)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movntpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_stream_pd (mem_addr : * mut f64 , a : __m128d) { crate :: arch :: asm ! (vps ! ("movntpd" , ",{a}") , p = in (reg) mem_addr , a = in (xmm_reg) a , options (nostack , preserves_flags) ,) ; }
}

macro_rules! _mm_store_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_sd_introspect!();
    # [doc = " Stores the lower 64 bits of a 128-bit vector of `[2 x double]` to a"] # [doc = " memory location."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movlps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_store_sd (mem_addr : * mut f64 , a : __m128d) { * mem_addr = simd_extract ! (a , 0) }
}

macro_rules! _mm_store_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_pd_introspect!();
    # [doc = " Stores 128-bits (composed of 2 packed double-precision (64-bit)"] # [doc = " floating-point elements) from `a` into memory. `mem_addr` must be aligned"] # [doc = " on a 16-byte boundary or a general-protection exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_store_pd (mem_addr : * mut f64 , a : __m128d) { * (mem_addr as * mut __m128d) = a ; }
}

macro_rules! _mm_storeu_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_pd_introspect!();
    # [doc = " Stores 128-bits (composed of 2 packed double-precision (64-bit)"] # [doc = " floating-point elements) from `a` into memory."] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storeu_pd (mem_addr : * mut f64 , a : __m128d) { mem_addr . cast :: < __m128d > () . write_unaligned (a) ; }
}

macro_rules! _mm_storeu_si16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_si16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_si16_introspect!();
    # [doc = " Store 16-bit integer from the first element of a into memory."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_si16)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _mm_storeu_si16 (mem_addr : * mut u8 , a : __m128i) { ptr :: write_unaligned (mem_addr as * mut i16 , simd_extract (a . as_i16x8 () , 0)) }
}

macro_rules! _mm_storeu_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_si32_introspect!();
    # [doc = " Store 32-bit integer from the first element of a into memory."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_si32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _mm_storeu_si32 (mem_addr : * mut u8 , a : __m128i) { ptr :: write_unaligned (mem_addr as * mut i32 , simd_extract (a . as_i32x4 () , 0)) }
}

macro_rules! _mm_storeu_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_si64_introspect!();
    # [doc = " Store 64-bit integer from the first element of a into memory."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_si64)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _mm_storeu_si64 (mem_addr : * mut u8 , a : __m128i) { ptr :: write_unaligned (mem_addr as * mut i64 , simd_extract (a . as_i64x2 () , 0)) }
}

macro_rules! _mm_store1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store1_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_store1_pd_introspect!();
    # [doc = " Stores the lower double-precision (64-bit) floating-point element from `a`"] # [doc = " into 2 contiguous elements in memory. `mem_addr` must be aligned on a"] # [doc = " 16-byte boundary or a general-protection exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store1_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_store1_pd (mem_addr : * mut f64 , a : __m128d) { let b : __m128d = simd_shuffle ! (a , a , [0 , 0]) ; * (mem_addr as * mut __m128d) = b ; }
}

macro_rules! _mm_store_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_pd1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_pd1_introspect!();
    # [doc = " Stores the lower double-precision (64-bit) floating-point element from `a`"] # [doc = " into 2 contiguous elements in memory. `mem_addr` must be aligned on a"] # [doc = " 16-byte boundary or a general-protection exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_pd1)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_store_pd1 (mem_addr : * mut f64 , a : __m128d) { let b : __m128d = simd_shuffle ! (a , a , [0 , 0]) ; * (mem_addr as * mut __m128d) = b ; }
}

macro_rules! _mm_storer_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storer_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_storer_pd_introspect!();
    # [doc = " Stores 2 double-precision (64-bit) floating-point elements from `a` into"] # [doc = " memory in reverse order."] # [doc = " `mem_addr` must be aligned on a 16-byte boundary or a general-protection"] # [doc = " exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storer_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_storer_pd (mem_addr : * mut f64 , a : __m128d) { let b : __m128d = simd_shuffle ! (a , a , [1 , 0]) ; * (mem_addr as * mut __m128d) = b ; }
}

macro_rules! _mm_storeh_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeh_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeh_pd_introspect!();
    # [doc = " Stores the upper 64 bits of a 128-bit vector of `[2 x double]` to a"] # [doc = " memory location."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeh_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storeh_pd (mem_addr : * mut f64 , a : __m128d) { * mem_addr = simd_extract ! (a , 1) ; }
}

macro_rules! _mm_storel_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storel_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_storel_pd_introspect!();
    # [doc = " Stores the lower 64 bits of a 128-bit vector of `[2 x double]` to a"] # [doc = " memory location."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storel_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movlps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storel_pd (mem_addr : * mut f64 , a : __m128d) { * mem_addr = simd_extract ! (a , 0) ; }
}

macro_rules! _mm_load1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load1_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_load1_pd_introspect!();
    # [doc = " Loads a double-precision (64-bit) floating-point element from memory"] # [doc = " into both elements of returned vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load1_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load1_pd (mem_addr : * const f64) -> __m128d { let d = * mem_addr ; _mm_setr_pd (d , d) }
}

macro_rules! _mm_load_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_pd1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_pd1_introspect!();
    # [doc = " Loads a double-precision (64-bit) floating-point element from memory"] # [doc = " into both elements of returned vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_pd1)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load_pd1 (mem_addr : * const f64) -> __m128d { _mm_load1_pd (mem_addr) }
}

macro_rules! _mm_loadr_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadr_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadr_pd_introspect!();
    # [doc = " Loads 2 double-precision (64-bit) floating-point elements from memory into"] # [doc = " the returned vector in reverse order. `mem_addr` must be aligned on a"] # [doc = " 16-byte boundary or a general-protection exception may be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadr_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadr_pd (mem_addr : * const f64) -> __m128d { let a = _mm_load_pd (mem_addr) ; simd_shuffle ! (a , a , [1 , 0]) }
}

macro_rules! _mm_loadu_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_pd_introspect!();
    # [doc = " Loads 128-bits (composed of 2 packed double-precision (64-bit)"] # [doc = " floating-point elements) from memory into the returned vector."] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadu_pd (mem_addr : * const f64) -> __m128d { let mut dst = _mm_undefined_pd () ; ptr :: copy_nonoverlapping (mem_addr as * const u8 , ptr :: addr_of_mut ! (dst) as * mut u8 , mem :: size_of :: < __m128d > () ,) ; dst }
}

macro_rules! _mm_loadu_si16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_si16 in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_si16_introspect!();
    # [doc = " Loads unaligned 16-bits of integer data from memory into new vector."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_si16)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _mm_loadu_si16 (mem_addr : * const u8) -> __m128i { transmute (i16x8 :: new (ptr :: read_unaligned (mem_addr as * const i16) , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,)) }
}

macro_rules! _mm_loadu_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_si32_introspect!();
    # [doc = " Loads unaligned 32-bits of integer data from memory into new vector."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_si32)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub unsafe fn _mm_loadu_si32 (mem_addr : * const u8) -> __m128i { transmute (i32x4 :: new (ptr :: read_unaligned (mem_addr as * const i32) , 0 , 0 , 0 ,)) }
}

macro_rules! _mm_loadu_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_si64_introspect!();
    # [doc = " Loads unaligned 64-bits of integer data from memory into new vector."] # [doc = ""] # [doc = " `mem_addr` does not need to be aligned on any particular boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_si64)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86_mm_loadu_si64" , since = "1.46.0")] pub unsafe fn _mm_loadu_si64 (mem_addr : * const u8) -> __m128i { transmute (i64x2 :: new (ptr :: read_unaligned (mem_addr as * const i64) , 0)) }
}

macro_rules! _mm_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_shuffle_pd_introspect!();
    # [doc = " Constructs a 128-bit floating-point vector of `[2 x double]` from two"] # [doc = " 128-bit vector parameters of `[2 x double]`, using the immediate-value"] # [doc = " parameter as a specifier."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shuffle_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (shufps , MASK = 2))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shuffle_pd < const MASK : i32 > (a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (MASK , 8) ; unsafe { simd_shuffle ! (a , b , [MASK as u32 & 0b1 , ((MASK as u32 >> 1) & 0b1) + 2]) } }
}

macro_rules! _mm_move_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_move_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_move_sd_introspect!();
    # [doc = " Constructs a 128-bit floating-point vector of `[2 x double]`. The lower"] # [doc = " 64 bits are set to the lower 64 bits of the second parameter. The upper"] # [doc = " 64 bits are set to the upper 64 bits of the first parameter."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_move_sd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_move_sd (a : __m128d , b : __m128d) -> __m128d { unsafe { _mm_setr_pd (simd_extract ! (b , 0) , simd_extract ! (a , 1)) } }
}

macro_rules! _mm_castpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castpd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_castpd_ps_introspect!();
    # [doc = " Casts a 128-bit floating-point vector of `[2 x double]` into a 128-bit"] # [doc = " floating-point vector of `[4 x float]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castpd_ps)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castpd_ps (a : __m128d) -> __m128 { unsafe { transmute (a) } }
}

macro_rules! _mm_castpd_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castpd_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_castpd_si128_introspect!();
    # [doc = " Casts a 128-bit floating-point vector of `[2 x double]` into a 128-bit"] # [doc = " integer vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castpd_si128)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castpd_si128 (a : __m128d) -> __m128i { unsafe { transmute (a) } }
}

macro_rules! _mm_castps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castps_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_castps_pd_introspect!();
    # [doc = " Casts a 128-bit floating-point vector of `[4 x float]` into a 128-bit"] # [doc = " floating-point vector of `[2 x double]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castps_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castps_pd (a : __m128) -> __m128d { unsafe { transmute (a) } }
}

macro_rules! _mm_castps_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castps_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_castps_si128_introspect!();
    # [doc = " Casts a 128-bit floating-point vector of `[4 x float]` into a 128-bit"] # [doc = " integer vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castps_si128)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castps_si128 (a : __m128) -> __m128i { unsafe { transmute (a) } }
}

macro_rules! _mm_castsi128_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castsi128_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_castsi128_pd_introspect!();
    # [doc = " Casts a 128-bit integer vector into a 128-bit floating-point vector"] # [doc = " of `[2 x double]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castsi128_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castsi128_pd (a : __m128i) -> __m128d { unsafe { transmute (a) } }
}

macro_rules! _mm_castsi128_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_castsi128_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_castsi128_ps_introspect!();
    # [doc = " Casts a 128-bit integer vector into a 128-bit floating-point vector"] # [doc = " of `[4 x float]`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_castsi128_ps)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_castsi128_ps (a : __m128i) -> __m128 { unsafe { transmute (a) } }
}

macro_rules! _mm_undefined_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_undefined_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_undefined_pd_introspect!();
    # [doc = " Returns vector of type __m128d with indeterminate elements.with indetermination elements."] # [doc = " Despite using the word \"undefined\" (following Intel's naming scheme), this non-deterministically"] # [doc = " picks some valid value and is not equivalent to [`mem::MaybeUninit`]."] # [doc = " In practice, this is typically equivalent to [`mem::zeroed`]."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_undefined_pd)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_undefined_pd () -> __m128d { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _mm_undefined_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_undefined_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_undefined_si128_introspect!();
    # [doc = " Returns vector of type __m128i with indeterminate elements.with indetermination elements."] # [doc = " Despite using the word \"undefined\" (following Intel's naming scheme), this non-deterministically"] # [doc = " picks some valid value and is not equivalent to [`mem::MaybeUninit`]."] # [doc = " In practice, this is typically equivalent to [`mem::zeroed`]."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_undefined_si128)"] # [inline] # [target_feature (enable = "sse2")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_undefined_si128 () -> __m128i { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _mm_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_pd_introspect!();
    # [doc = " The resulting `__m128d` element is composed by the low-order values of"] # [doc = " the two `__m128d` interleaved input elements, i.e.:"] # [doc = ""] # [doc = " * The `[127:64]` bits are copied from the `[127:64]` bits of the second input"] # [doc = " * The `[63:0]` bits are copied from the `[127:64]` bits of the first input"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (unpckhpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_shuffle ! (a , b , [1 , 3]) } }
}

macro_rules! _mm_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_pd_introspect!();
    # [doc = " The resulting `__m128d` element is composed by the high-order values of"] # [doc = " the two `__m128d` interleaved input elements, i.e.:"] # [doc = ""] # [doc = " * The `[127:64]` bits are copied from the `[63:0]` bits of the second input"] # [doc = " * The `[63:0]` bits are copied from the `[63:0]` bits of the first input"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_pd)"] # [inline] # [target_feature (enable = "sse2")] # [cfg_attr (test , assert_instr (movlhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { simd_shuffle ! (a , b , [0 , 2]) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse2.pause"] fn pause () ; # [link_name = "llvm.x86.sse2.clflush"] fn clflush (p : * const u8) ; # [link_name = "llvm.x86.sse2.lfence"] fn lfence () ; # [link_name = "llvm.x86.sse2.mfence"] fn mfence () ; # [link_name = "llvm.x86.sse2.pmadd.wd"] fn pmaddwd (a : i16x8 , b : i16x8) -> i32x4 ; # [link_name = "llvm.x86.sse2.psad.bw"] fn psadbw (a : u8x16 , b : u8x16) -> u64x2 ; # [link_name = "llvm.x86.sse2.psll.w"] fn psllw (a : i16x8 , count : i16x8) -> i16x8 ; # [link_name = "llvm.x86.sse2.psll.d"] fn pslld (a : i32x4 , count : i32x4) -> i32x4 ; # [link_name = "llvm.x86.sse2.psll.q"] fn psllq (a : i64x2 , count : i64x2) -> i64x2 ; # [link_name = "llvm.x86.sse2.psra.w"] fn psraw (a : i16x8 , count : i16x8) -> i16x8 ; # [link_name = "llvm.x86.sse2.psra.d"] fn psrad (a : i32x4 , count : i32x4) -> i32x4 ; # [link_name = "llvm.x86.sse2.psrl.w"] fn psrlw (a : i16x8 , count : i16x8) -> i16x8 ; # [link_name = "llvm.x86.sse2.psrl.d"] fn psrld (a : i32x4 , count : i32x4) -> i32x4 ; # [link_name = "llvm.x86.sse2.psrl.q"] fn psrlq (a : i64x2 , count : i64x2) -> i64x2 ; # [link_name = "llvm.x86.sse2.cvtps2dq"] fn cvtps2dq (a : __m128) -> i32x4 ; # [link_name = "llvm.x86.sse2.maskmov.dqu"] fn maskmovdqu (a : i8x16 , mask : i8x16 , mem_addr : * mut i8) ; # [link_name = "llvm.x86.sse2.packsswb.128"] fn packsswb (a : i16x8 , b : i16x8) -> i8x16 ; # [link_name = "llvm.x86.sse2.packssdw.128"] fn packssdw (a : i32x4 , b : i32x4) -> i16x8 ; # [link_name = "llvm.x86.sse2.packuswb.128"] fn packuswb (a : i16x8 , b : i16x8) -> u8x16 ; # [link_name = "llvm.x86.sse2.max.sd"] fn maxsd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse2.max.pd"] fn maxpd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse2.min.sd"] fn minsd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse2.min.pd"] fn minpd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse2.cmp.sd"] fn cmpsd (a : __m128d , b : __m128d , imm8 : i8) -> __m128d ; # [link_name = "llvm.x86.sse2.cmp.pd"] fn cmppd (a : __m128d , b : __m128d , imm8 : i8) -> __m128d ; # [link_name = "llvm.x86.sse2.comieq.sd"] fn comieqsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.comilt.sd"] fn comiltsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.comile.sd"] fn comilesd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.comigt.sd"] fn comigtsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.comige.sd"] fn comigesd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.comineq.sd"] fn comineqsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomieq.sd"] fn ucomieqsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomilt.sd"] fn ucomiltsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomile.sd"] fn ucomilesd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomigt.sd"] fn ucomigtsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomige.sd"] fn ucomigesd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.ucomineq.sd"] fn ucomineqsd (a : __m128d , b : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.cvtpd2dq"] fn cvtpd2dq (a : __m128d) -> i32x4 ; # [link_name = "llvm.x86.sse2.cvtsd2si"] fn cvtsd2si (a : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.cvtsd2ss"] fn cvtsd2ss (a : __m128 , b : __m128d) -> __m128 ; # [link_name = "llvm.x86.sse2.cvtss2sd"] fn cvtss2sd (a : __m128d , b : __m128) -> __m128d ; # [link_name = "llvm.x86.sse2.cvttpd2dq"] fn cvttpd2dq (a : __m128d) -> i32x4 ; # [link_name = "llvm.x86.sse2.cvttsd2si"] fn cvttsd2si (a : __m128d) -> i32 ; # [link_name = "llvm.x86.sse2.cvttps2dq"] fn cvttps2dq (a : __m128) -> i32x4 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , hint :: black_box , } ;}
mkuse!{use std :: { boxed , f32 , f64 , mem :: { self , transmute } , ptr , } ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{const NAN : f64 = f64 :: NAN ;}

macro_rules! test_mm_pause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_pause in module {}", module_path!());
    };
}

mkfn!{
    test_mm_pause_introspect!();
    # [test] fn test_mm_pause () { unsafe { _mm_pause () } }
}

macro_rules! test_mm_clflush_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_clflush in module {}", module_path!());
    };
}

mkfn!{
    test_mm_clflush_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_clflush () { let x = 0_u8 ; _mm_clflush (ptr :: addr_of ! (x)) ; }
}

macro_rules! test_mm_lfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_lfence in module {}", module_path!());
    };
}

mkfn!{
    test_mm_lfence_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_lfence () { _mm_lfence () ; }
}

macro_rules! test_mm_mfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mfence in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mfence_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_mfence () { _mm_mfence () ; }
}

macro_rules! test_mm_add_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_epi8 () { let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ; let r = _mm_add_epi8 (a , b) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (16 , 18 , 20 , 22 , 24 , 26 , 28 , 30 , 32 , 34 , 36 , 38 , 40 , 42 , 44 , 46 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_add_epi8_overflow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_epi8_overflow in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_epi8_overflow_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_epi8_overflow () { let a = _mm_set1_epi8 (0x7F) ; let b = _mm_set1_epi8 (1) ; let r = _mm_add_epi8 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (- 128)) ; }
}

macro_rules! test_mm_add_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_add_epi16 (a , b) ; let e = _mm_setr_epi16 (8 , 10 , 12 , 14 , 16 , 18 , 20 , 22) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_add_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_epi32 () { let a = _mm_setr_epi32 (0 , 1 , 2 , 3) ; let b = _mm_setr_epi32 (4 , 5 , 6 , 7) ; let r = _mm_add_epi32 (a , b) ; let e = _mm_setr_epi32 (4 , 6 , 8 , 10) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_epi64 () { let a = _mm_setr_epi64x (0 , 1) ; let b = _mm_setr_epi64x (2 , 3) ; let r = _mm_add_epi64 (a , b) ; let e = _mm_setr_epi64x (2 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_adds_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi8 () { let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ; let r = _mm_adds_epi8 (a , b) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (16 , 18 , 20 , 22 , 24 , 26 , 28 , 30 , 32 , 34 , 36 , 38 , 40 , 42 , 44 , 46 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_adds_epi8_saturate_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi8_saturate_positive in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi8_saturate_positive_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi8_saturate_positive () { let a = _mm_set1_epi8 (0x7F) ; let b = _mm_set1_epi8 (1) ; let r = _mm_adds_epi8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_adds_epi8_saturate_negative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi8_saturate_negative in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi8_saturate_negative_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi8_saturate_negative () { let a = _mm_set1_epi8 (- 0x80) ; let b = _mm_set1_epi8 (- 1) ; let r = _mm_adds_epi8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_adds_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_adds_epi16 (a , b) ; let e = _mm_setr_epi16 (8 , 10 , 12 , 14 , 16 , 18 , 20 , 22) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_adds_epi16_saturate_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi16_saturate_positive in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi16_saturate_positive_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi16_saturate_positive () { let a = _mm_set1_epi16 (0x7FFF) ; let b = _mm_set1_epi16 (1) ; let r = _mm_adds_epi16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_adds_epi16_saturate_negative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epi16_saturate_negative in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epi16_saturate_negative_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epi16_saturate_negative () { let a = _mm_set1_epi16 (- 0x8000) ; let b = _mm_set1_epi16 (- 1) ; let r = _mm_adds_epi16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_adds_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epu8 () { let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ; let r = _mm_adds_epu8 (a , b) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (16 , 18 , 20 , 22 , 24 , 26 , 28 , 30 , 32 , 34 , 36 , 38 , 40 , 42 , 44 , 46 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_adds_epu8_saturate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epu8_saturate in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epu8_saturate_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epu8_saturate () { let a = _mm_set1_epi8 (! 0) ; let b = _mm_set1_epi8 (1) ; let r = _mm_adds_epu8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_adds_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epu16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epu16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epu16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_adds_epu16 (a , b) ; let e = _mm_setr_epi16 (8 , 10 , 12 , 14 , 16 , 18 , 20 , 22) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_adds_epu16_saturate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_adds_epu16_saturate in module {}", module_path!());
    };
}

mkfn!{
    test_mm_adds_epu16_saturate_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_adds_epu16_saturate () { let a = _mm_set1_epi16 (! 0) ; let b = _mm_set1_epi16 (1) ; let r = _mm_adds_epu16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_avg_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_avg_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_avg_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_avg_epu8 () { let (a , b) = (_mm_set1_epi8 (3) , _mm_set1_epi8 (9)) ; let r = _mm_avg_epu8 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (6)) ; }
}

macro_rules! test_mm_avg_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_avg_epu16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_avg_epu16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_avg_epu16 () { let (a , b) = (_mm_set1_epi16 (3) , _mm_set1_epi16 (9)) ; let r = _mm_avg_epu16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (6)) ; }
}

macro_rules! test_mm_madd_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_madd_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_madd_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_madd_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_setr_epi16 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm_madd_epi16 (a , b) ; let e = _mm_setr_epi32 (29 , 81 , 149 , 233) ; assert_eq_m128i (r , e) ; let a = _mm_setr_epi16 (i16 :: MAX , i16 :: MAX , i16 :: MIN , i16 :: MIN , i16 :: MIN , i16 :: MAX , 0 , 0 ,) ; let b = _mm_setr_epi16 (i16 :: MAX , i16 :: MAX , i16 :: MIN , i16 :: MIN , i16 :: MAX , i16 :: MIN , 0 , 0 ,) ; let r = _mm_madd_epi16 (a , b) ; let e = _mm_setr_epi32 (0x7FFE0002 , i32 :: MIN , - 0x7FFF0000 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_max_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_max_epi16 () { let a = _mm_set1_epi16 (1) ; let b = _mm_set1_epi16 (- 1) ; let r = _mm_max_epi16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_max_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_max_epu8 () { let a = _mm_set1_epi8 (1) ; let b = _mm_set1_epi8 (! 0) ; let r = _mm_max_epu8 (a , b) ; assert_eq_m128i (r , b) ; }
}

macro_rules! test_mm_min_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_min_epi16 () { let a = _mm_set1_epi16 (1) ; let b = _mm_set1_epi16 (- 1) ; let r = _mm_min_epi16 (a , b) ; assert_eq_m128i (r , b) ; }
}

macro_rules! test_mm_min_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_min_epu8 () { let a = _mm_set1_epi8 (1) ; let b = _mm_set1_epi8 (! 0) ; let r = _mm_min_epu8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_mulhi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mulhi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mulhi_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mulhi_epi16 () { let (a , b) = (_mm_set1_epi16 (1000) , _mm_set1_epi16 (- 1001)) ; let r = _mm_mulhi_epi16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (- 16)) ; }
}

macro_rules! test_mm_mulhi_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mulhi_epu16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mulhi_epu16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mulhi_epu16 () { let (a , b) = (_mm_set1_epi16 (1000) , _mm_set1_epi16 (1001)) ; let r = _mm_mulhi_epu16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (15)) ; }
}

macro_rules! test_mm_mullo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mullo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mullo_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mullo_epi16 () { let (a , b) = (_mm_set1_epi16 (1000) , _mm_set1_epi16 (- 1001)) ; let r = _mm_mullo_epi16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (- 17960)) ; }
}

macro_rules! test_mm_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mul_epu32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mul_epu32 () { let a = _mm_setr_epi64x (1_000_000_000 , 1 << 34) ; let b = _mm_setr_epi64x (1_000_000_000 , 1 << 35) ; let r = _mm_mul_epu32 (a , b) ; let e = _mm_setr_epi64x (1_000_000_000 * 1_000_000_000 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_sad_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sad_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sad_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sad_epu8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (255u8 as i8 , 254u8 as i8 , 253u8 as i8 , 252u8 as i8 , 1 , 2 , 3 , 4 , 155u8 as i8 , 154u8 as i8 , 153u8 as i8 , 152u8 as i8 , 1 , 2 , 3 , 4 ,) ; let b = _mm_setr_epi8 (0 , 0 , 0 , 0 , 2 , 1 , 2 , 1 , 1 , 1 , 1 , 1 , 1 , 2 , 1 , 2) ; let r = _mm_sad_epu8 (a , b) ; let e = _mm_setr_epi64x (1020 , 614) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_sub_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_epi8 () { let (a , b) = (_mm_set1_epi8 (5) , _mm_set1_epi8 (6)) ; let r = _mm_sub_epi8 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (- 1)) ; }
}

macro_rules! test_mm_sub_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_epi16 () { let (a , b) = (_mm_set1_epi16 (5) , _mm_set1_epi16 (6)) ; let r = _mm_sub_epi16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (- 1)) ; }
}

macro_rules! test_mm_sub_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_epi32 () { let (a , b) = (_mm_set1_epi32 (5) , _mm_set1_epi32 (6)) ; let r = _mm_sub_epi32 (a , b) ; assert_eq_m128i (r , _mm_set1_epi32 (- 1)) ; }
}

macro_rules! test_mm_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_epi64 () { let (a , b) = (_mm_set1_epi64x (5) , _mm_set1_epi64x (6)) ; let r = _mm_sub_epi64 (a , b) ; assert_eq_m128i (r , _mm_set1_epi64x (- 1)) ; }
}

macro_rules! test_mm_subs_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi8 () { let (a , b) = (_mm_set1_epi8 (5) , _mm_set1_epi8 (2)) ; let r = _mm_subs_epi8 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (3)) ; }
}

macro_rules! test_mm_subs_epi8_saturate_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi8_saturate_positive in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi8_saturate_positive_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi8_saturate_positive () { let a = _mm_set1_epi8 (0x7F) ; let b = _mm_set1_epi8 (- 1) ; let r = _mm_subs_epi8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_subs_epi8_saturate_negative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi8_saturate_negative in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi8_saturate_negative_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi8_saturate_negative () { let a = _mm_set1_epi8 (- 0x80) ; let b = _mm_set1_epi8 (1) ; let r = _mm_subs_epi8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_subs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi16 () { let (a , b) = (_mm_set1_epi16 (5) , _mm_set1_epi16 (2)) ; let r = _mm_subs_epi16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (3)) ; }
}

macro_rules! test_mm_subs_epi16_saturate_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi16_saturate_positive in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi16_saturate_positive_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi16_saturate_positive () { let a = _mm_set1_epi16 (0x7FFF) ; let b = _mm_set1_epi16 (- 1) ; let r = _mm_subs_epi16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_subs_epi16_saturate_negative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epi16_saturate_negative in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epi16_saturate_negative_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epi16_saturate_negative () { let a = _mm_set1_epi16 (- 0x8000) ; let b = _mm_set1_epi16 (1) ; let r = _mm_subs_epi16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_subs_epu8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epu8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epu8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epu8 () { let (a , b) = (_mm_set1_epi8 (5) , _mm_set1_epi8 (2)) ; let r = _mm_subs_epu8 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (3)) ; }
}

macro_rules! test_mm_subs_epu8_saturate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epu8_saturate in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epu8_saturate_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epu8_saturate () { let a = _mm_set1_epi8 (0) ; let b = _mm_set1_epi8 (1) ; let r = _mm_subs_epu8 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_subs_epu16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epu16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epu16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epu16 () { let (a , b) = (_mm_set1_epi16 (5) , _mm_set1_epi16 (2)) ; let r = _mm_subs_epu16 (a , b) ; assert_eq_m128i (r , _mm_set1_epi16 (3)) ; }
}

macro_rules! test_mm_subs_epu16_saturate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_subs_epu16_saturate in module {}", module_path!());
    };
}

mkfn!{
    test_mm_subs_epu16_saturate_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_subs_epu16_saturate () { let a = _mm_set1_epi16 (0) ; let b = _mm_set1_epi16 (1) ; let r = _mm_subs_epu16 (a , b) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_slli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_slli_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_slli_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_slli_si128 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_slli_si128 :: < 1 > (a) ; let e = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_slli_si128 :: < 15 > (a) ; let e = _mm_setr_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1) ; assert_eq_m128i (r , e) ; # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_slli_si128 :: < 16 > (a) ; assert_eq_m128i (r , _mm_set1_epi8 (0)) ; }
}

macro_rules! test_mm_slli_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_slli_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_slli_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_slli_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_slli_epi16 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi16 (0xCC0 , - 0xCC0 , 0xDD0 , - 0xDD0 , 0xEE0 , - 0xEE0 , 0xFF0 , - 0xFF0) ,) ; let r = _mm_slli_epi16 :: < 16 > (a) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; }
}

macro_rules! test_mm_sll_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sll_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sll_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sll_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_sll_epi16 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi16 (0xCC0 , - 0xCC0 , 0xDD0 , - 0xDD0 , 0xEE0 , - 0xEE0 , 0xFF0 , - 0xFF0) ,) ; let r = _mm_sll_epi16 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_sll_epi16 (a , _mm_set_epi64x (0 , 16)) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; let r = _mm_sll_epi16 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; }
}

macro_rules! test_mm_slli_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_slli_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_slli_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_slli_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_slli_epi32 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEEE0 , - 0xEEEE0 , 0xFFFF0 , - 0xFFFF0)) ; let r = _mm_slli_epi32 :: < 32 > (a) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; }
}

macro_rules! test_mm_sll_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sll_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sll_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sll_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_sll_epi32 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEEE0 , - 0xEEEE0 , 0xFFFF0 , - 0xFFFF0)) ; let r = _mm_sll_epi32 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_sll_epi32 (a , _mm_set_epi64x (0 , 32)) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; let r = _mm_sll_epi32 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; }
}

macro_rules! test_mm_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_slli_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_slli_epi64 () { let a = _mm_set_epi64x (0xFFFFFFFF , - 0xFFFFFFFF) ; let r = _mm_slli_epi64 :: < 4 > (a) ; assert_eq_m128i (r , _mm_set_epi64x (0xFFFFFFFF0 , - 0xFFFFFFFF0)) ; let r = _mm_slli_epi64 :: < 64 > (a) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; }
}

macro_rules! test_mm_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sll_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sll_epi64 () { let a = _mm_set_epi64x (0xFFFFFFFF , - 0xFFFFFFFF) ; let r = _mm_sll_epi64 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_set_epi64x (0xFFFFFFFF0 , - 0xFFFFFFFF0)) ; let r = _mm_sll_epi64 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_sll_epi64 (a , _mm_set_epi64x (0 , 64)) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; let r = _mm_sll_epi64 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; }
}

macro_rules! test_mm_srai_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srai_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srai_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srai_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_srai_epi16 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi16 (0xC , - 0xD , 0xD , - 0xE , 0xE , - 0xF , 0xF , - 0x10) ,) ; let r = _mm_srai_epi16 :: < 16 > (a) ; assert_eq_m128i (r , _mm_setr_epi16 (0 , - 1 , 0 , - 1 , 0 , - 1 , 0 , - 1)) ; }
}

macro_rules! test_mm_sra_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sra_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sra_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sra_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_sra_epi16 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi16 (0xC , - 0xD , 0xD , - 0xE , 0xE , - 0xF , 0xF , - 0x10) ,) ; let r = _mm_sra_epi16 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_sra_epi16 (a , _mm_set_epi64x (0 , 16)) ; assert_eq_m128i (r , _mm_setr_epi16 (0 , - 1 , 0 , - 1 , 0 , - 1 , 0 , - 1)) ; let r = _mm_sra_epi16 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_setr_epi16 (0 , - 1 , 0 , - 1 , 0 , - 1 , 0 , - 1)) ; }
}

macro_rules! test_mm_srai_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srai_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srai_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srai_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_srai_epi32 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEE , - 0xEEF , 0xFFF , - 0x1000)) ; let r = _mm_srai_epi32 :: < 32 > (a) ; assert_eq_m128i (r , _mm_setr_epi32 (0 , - 1 , 0 , - 1)) ; }
}

macro_rules! test_mm_sra_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sra_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sra_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sra_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_sra_epi32 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEE , - 0xEEF , 0xFFF , - 0x1000)) ; let r = _mm_sra_epi32 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_sra_epi32 (a , _mm_set_epi64x (0 , 32)) ; assert_eq_m128i (r , _mm_setr_epi32 (0 , - 1 , 0 , - 1)) ; let r = _mm_sra_epi32 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_setr_epi32 (0 , - 1 , 0 , - 1)) ; }
}

macro_rules! test_mm_srli_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srli_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srli_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srli_si128 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_srli_si128 :: < 1 > (a) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 0 ,) ; assert_eq_m128i (r , e) ; # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_srli_si128 :: < 15 > (a) ; let e = _mm_setr_epi8 (16 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_srli_si128 :: < 16 > (a) ; assert_eq_m128i (r , _mm_set1_epi8 (0)) ; }
}

macro_rules! test_mm_srli_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srli_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srli_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srli_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_srli_epi16 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi16 (0xC , 0xFF3 , 0xD , 0xFF2 , 0xE , 0xFF1 , 0xF , 0xFF0) ,) ; let r = _mm_srli_epi16 :: < 16 > (a) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; }
}

macro_rules! test_mm_srl_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srl_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srl_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srl_epi16 () { let a = _mm_setr_epi16 (0xCC , - 0xCC , 0xDD , - 0xDD , 0xEE , - 0xEE , 0xFF , - 0xFF) ; let r = _mm_srl_epi16 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi16 (0xC , 0xFF3 , 0xD , 0xFF2 , 0xE , 0xFF1 , 0xF , 0xFF0) ,) ; let r = _mm_srl_epi16 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_srl_epi16 (a , _mm_set_epi64x (0 , 16)) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; let r = _mm_srl_epi16 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi16 (0)) ; }
}

macro_rules! test_mm_srli_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srli_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srli_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srli_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_srli_epi32 :: < 4 > (a) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEE , 0xFFFF111 , 0xFFF , 0xFFFF000)) ; let r = _mm_srli_epi32 :: < 32 > (a) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; }
}

macro_rules! test_mm_srl_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srl_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srl_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srl_epi32 () { let a = _mm_setr_epi32 (0xEEEE , - 0xEEEE , 0xFFFF , - 0xFFFF) ; let r = _mm_srl_epi32 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_setr_epi32 (0xEEE , 0xFFFF111 , 0xFFF , 0xFFFF000)) ; let r = _mm_srl_epi32 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_srl_epi32 (a , _mm_set_epi64x (0 , 32)) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; let r = _mm_srl_epi32 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi32 (0)) ; }
}

macro_rules! test_mm_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srli_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srli_epi64 () { let a = _mm_set_epi64x (0xFFFFFFFF , - 0xFFFFFFFF) ; let r = _mm_srli_epi64 :: < 4 > (a) ; assert_eq_m128i (r , _mm_set_epi64x (0xFFFFFFF , 0xFFFFFFFF0000000)) ; let r = _mm_srli_epi64 :: < 64 > (a) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; }
}

macro_rules! test_mm_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srl_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_srl_epi64 () { let a = _mm_set_epi64x (0xFFFFFFFF , - 0xFFFFFFFF) ; let r = _mm_srl_epi64 (a , _mm_set_epi64x (0 , 4)) ; assert_eq_m128i (r , _mm_set_epi64x (0xFFFFFFF , 0xFFFFFFFF0000000)) ; let r = _mm_srl_epi64 (a , _mm_set_epi64x (4 , 0)) ; assert_eq_m128i (r , a) ; let r = _mm_srl_epi64 (a , _mm_set_epi64x (0 , 64)) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; let r = _mm_srl_epi64 (a , _mm_set_epi64x (0 , i64 :: MAX)) ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; }
}

macro_rules! test_mm_and_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_and_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_and_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_and_si128 () { let a = _mm_set1_epi8 (5) ; let b = _mm_set1_epi8 (3) ; let r = _mm_and_si128 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (1)) ; }
}

macro_rules! test_mm_andnot_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_andnot_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_andnot_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_andnot_si128 () { let a = _mm_set1_epi8 (5) ; let b = _mm_set1_epi8 (3) ; let r = _mm_andnot_si128 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (2)) ; }
}

macro_rules! test_mm_or_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_or_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_or_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_or_si128 () { let a = _mm_set1_epi8 (5) ; let b = _mm_set1_epi8 (3) ; let r = _mm_or_si128 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (7)) ; }
}

macro_rules! test_mm_xor_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_xor_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_xor_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_xor_si128 () { let a = _mm_set1_epi8 (5) ; let b = _mm_set1_epi8 (3) ; let r = _mm_xor_si128 (a , b) ; assert_eq_m128i (r , _mm_set1_epi8 (6)) ; }
}

macro_rules! test_mm_cmpeq_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpeq_epi8 () { let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let b = _mm_setr_epi8 (15 , 14 , 2 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm_cmpeq_epi8 (a , b) ; # [rustfmt :: skip] assert_eq_m128i (r , _mm_setr_epi8 (0 , 0 , 0xFFu8 as i8 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_cmpeq_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpeq_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (7 , 6 , 2 , 4 , 3 , 2 , 1 , 0) ; let r = _mm_cmpeq_epi16 (a , b) ; assert_eq_m128i (r , _mm_setr_epi16 (0 , 0 , ! 0 , 0 , 0 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_cmpeq_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpeq_epi32 () { let a = _mm_setr_epi32 (0 , 1 , 2 , 3) ; let b = _mm_setr_epi32 (3 , 2 , 2 , 0) ; let r = _mm_cmpeq_epi32 (a , b) ; assert_eq_m128i (r , _mm_setr_epi32 (0 , 0 , ! 0 , 0)) ; }
}

macro_rules! test_mm_cmpgt_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpgt_epi8 () { let a = _mm_set_epi8 (5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; let b = _mm_set1_epi8 (0) ; let r = _mm_cmpgt_epi8 (a , b) ; let e = _mm_set_epi8 (! 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpgt_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpgt_epi16 () { let a = _mm_set_epi16 (5 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; let b = _mm_set1_epi16 (0) ; let r = _mm_cmpgt_epi16 (a , b) ; let e = _mm_set_epi16 (! 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpgt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpgt_epi32 () { let a = _mm_set_epi32 (5 , 0 , 0 , 0) ; let b = _mm_set1_epi32 (0) ; let r = _mm_cmpgt_epi32 (a , b) ; assert_eq_m128i (r , _mm_set_epi32 (! 0 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_cmplt_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmplt_epi8 () { let a = _mm_set1_epi8 (0) ; let b = _mm_set_epi8 (5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; let r = _mm_cmplt_epi8 (a , b) ; let e = _mm_set_epi8 (! 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmplt_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmplt_epi16 () { let a = _mm_set1_epi16 (0) ; let b = _mm_set_epi16 (5 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; let r = _mm_cmplt_epi16 (a , b) ; let e = _mm_set_epi16 (! 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmplt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmplt_epi32 () { let a = _mm_set1_epi32 (0) ; let b = _mm_set_epi32 (5 , 0 , 0 , 0) ; let r = _mm_cmplt_epi32 (a , b) ; assert_eq_m128i (r , _mm_set_epi32 (! 0 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi32_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtepi32_pd () { let a = _mm_set_epi32 (35 , 25 , 15 , 5) ; let r = _mm_cvtepi32_pd (a) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 15.0)) ; }
}

macro_rules! test_mm_cvtsi32_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi32_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi32_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi32_sd () { let a = _mm_set1_pd (3.5) ; let r = _mm_cvtsi32_sd (a , 5) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 3.5)) ; }
}

macro_rules! test_mm_cvtepi32_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi32_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi32_ps_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtepi32_ps () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let r = _mm_cvtepi32_ps (a) ; assert_eq_m128 (r , _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0)) ; }
}

macro_rules! test_mm_cvtps_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtps_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtps_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtps_epi32 () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let r = _mm_cvtps_epi32 (a) ; assert_eq_m128i (r , _mm_setr_epi32 (1 , 2 , 3 , 4)) ; }
}

macro_rules! test_mm_cvtsi32_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi32_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi32_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi32_si128 () { let r = _mm_cvtsi32_si128 (5) ; assert_eq_m128i (r , _mm_setr_epi32 (5 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_cvtsi128_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi128_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi128_si32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsi128_si32 () { let r = _mm_cvtsi128_si32 (_mm_setr_epi32 (5 , 0 , 0 , 0)) ; assert_eq ! (r , 5) ; }
}

macro_rules! test_mm_set_epi64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_epi64x in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_epi64x_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_epi64x () { let r = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , _mm_setr_epi64x (1 , 0)) ; }
}

macro_rules! test_mm_set_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_epi32 () { let r = _mm_set_epi32 (0 , 1 , 2 , 3) ; assert_eq_m128i (r , _mm_setr_epi32 (3 , 2 , 1 , 0)) ; }
}

macro_rules! test_mm_set_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_epi16 () { let r = _mm_set_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m128i (r , _mm_setr_epi16 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0)) ; }
}

macro_rules! test_mm_set_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_epi8 () { # [rustfmt :: skip] let r = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_set1_epi64x_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_epi64x in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_epi64x_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set1_epi64x () { let r = _mm_set1_epi64x (1) ; assert_eq_m128i (r , _mm_set1_epi64x (1)) ; }
}

macro_rules! test_mm_set1_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set1_epi32 () { let r = _mm_set1_epi32 (1) ; assert_eq_m128i (r , _mm_set1_epi32 (1)) ; }
}

macro_rules! test_mm_set1_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set1_epi16 () { let r = _mm_set1_epi16 (1) ; assert_eq_m128i (r , _mm_set1_epi16 (1)) ; }
}

macro_rules! test_mm_set1_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set1_epi8 () { let r = _mm_set1_epi8 (1) ; assert_eq_m128i (r , _mm_set1_epi8 (1)) ; }
}

macro_rules! test_mm_setr_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setr_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setr_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setr_epi32 () { let r = _mm_setr_epi32 (0 , 1 , 2 , 3) ; assert_eq_m128i (r , _mm_setr_epi32 (0 , 1 , 2 , 3)) ; }
}

macro_rules! test_mm_setr_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setr_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setr_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setr_epi16 () { let r = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m128i (r , _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; }
}

macro_rules! test_mm_setr_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setr_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setr_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setr_epi8 () { # [rustfmt :: skip] let r = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_setzero_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setzero_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setzero_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setzero_si128 () { let r = _mm_setzero_si128 () ; assert_eq_m128i (r , _mm_set1_epi64x (0)) ; }
}

macro_rules! test_mm_loadl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadl_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadl_epi64 () { let a = _mm_setr_epi64x (6 , 5) ; let r = _mm_loadl_epi64 (ptr :: addr_of ! (a)) ; assert_eq_m128i (r , _mm_setr_epi64x (6 , 0)) ; }
}

macro_rules! test_mm_load_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_load_si128 () { let a = _mm_set_epi64x (5 , 6) ; let r = _mm_load_si128 (ptr :: addr_of ! (a) as * const _) ; assert_eq_m128i (a , r) ; }
}

macro_rules! test_mm_loadu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadu_si128 () { let a = _mm_set_epi64x (5 , 6) ; let r = _mm_loadu_si128 (ptr :: addr_of ! (a) as * const _) ; assert_eq_m128i (a , r) ; }
}

macro_rules! test_mm_maskmoveu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskmoveu_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskmoveu_si128_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_maskmoveu_si128 () { let a = _mm_set1_epi8 (9) ; # [rustfmt :: skip] let mask = _mm_set_epi8 (0 , 0 , 0x80u8 as i8 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,) ; let mut r = _mm_set1_epi8 (0) ; _mm_maskmoveu_si128 (a , mask , ptr :: addr_of_mut ! (r) as * mut i8) ; let e = _mm_set_epi8 (0 , 0 , 9 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_store_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_store_si128 () { let a = _mm_set1_epi8 (9) ; let mut r = _mm_set1_epi8 (0) ; _mm_store_si128 (& mut r , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_storeu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeu_si128 () { let a = _mm_set1_epi8 (9) ; let mut r = _mm_set1_epi8 (0) ; _mm_storeu_si128 (& mut r , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_storel_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storel_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storel_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storel_epi64 () { let a = _mm_setr_epi64x (2 , 9) ; let mut r = _mm_set1_epi8 (0) ; _mm_storel_epi64 (& mut r , a) ; assert_eq_m128i (r , _mm_setr_epi64x (2 , 0)) ; }
}

macro_rules! test_mm_stream_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_si128_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_si128 () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let mut r = _mm_undefined_si128 () ; _mm_stream_si128 (ptr :: addr_of_mut ! (r) , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_stream_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_si32_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_si32 () { let a : i32 = 7 ; let mut mem = boxed :: Box :: < i32 > :: new (- 1) ; _mm_stream_si32 (ptr :: addr_of_mut ! (* mem) , a) ; assert_eq ! (a , * mem) ; }
}

macro_rules! test_mm_move_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_move_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_move_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_move_epi64 () { let a = _mm_setr_epi64x (5 , 6) ; let r = _mm_move_epi64 (a) ; assert_eq_m128i (r , _mm_setr_epi64x (5 , 0)) ; }
}

macro_rules! test_mm_packs_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_packs_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_packs_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_packs_epi16 () { let a = _mm_setr_epi16 (0x80 , - 0x81 , 0 , 0 , 0 , 0 , 0 , 0) ; let b = _mm_setr_epi16 (0 , 0 , 0 , 0 , 0 , 0 , - 0x81 , 0x80) ; let r = _mm_packs_epi16 (a , b) ; # [rustfmt :: skip] assert_eq_m128i (r , _mm_setr_epi8 (0x7F , - 0x80 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 0x80 , 0x7F)) ; }
}

macro_rules! test_mm_packs_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_packs_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_packs_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_packs_epi32 () { let a = _mm_setr_epi32 (0x8000 , - 0x8001 , 0 , 0) ; let b = _mm_setr_epi32 (0 , 0 , - 0x8001 , 0x8000) ; let r = _mm_packs_epi32 (a , b) ; assert_eq_m128i (r , _mm_setr_epi16 (0x7FFF , - 0x8000 , 0 , 0 , 0 , 0 , - 0x8000 , 0x7FFF) ,) ; }
}

macro_rules! test_mm_packus_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_packus_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_packus_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_packus_epi16 () { let a = _mm_setr_epi16 (0x100 , - 1 , 0 , 0 , 0 , 0 , 0 , 0) ; let b = _mm_setr_epi16 (0 , 0 , 0 , 0 , 0 , 0 , - 1 , 0x100) ; let r = _mm_packus_epi16 (a , b) ; assert_eq_m128i (r , _mm_setr_epi8 (! 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , ! 0) ,) ; }
}

macro_rules! test_mm_extract_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_extract_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_extract_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_extract_epi16 () { let a = _mm_setr_epi16 (- 1 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r1 = _mm_extract_epi16 :: < 0 > (a) ; let r2 = _mm_extract_epi16 :: < 3 > (a) ; assert_eq ! (r1 , 0xFFFF) ; assert_eq ! (r2 , 3) ; }
}

macro_rules! test_mm_insert_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_insert_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_insert_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_insert_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm_insert_epi16 :: < 0 > (a , 9) ; let e = _mm_setr_epi16 (9 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_movemask_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movemask_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movemask_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_movemask_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (0b1000_0000u8 as i8 , 0b0 , 0b1000_0000u8 as i8 , 0b01 , 0b0101 , 0b1111_0000u8 as i8 , 0 , 0 , 0 , 0b1011_0101u8 as i8 , 0b1111_0000u8 as i8 , 0b0101 , 0b01 , 0b1000_0000u8 as i8 , 0b0 , 0b1000_0000u8 as i8 ,) ; let r = _mm_movemask_epi8 (a) ; assert_eq ! (r , 0b10100110_00100101) ; }
}

macro_rules! test_mm_shuffle_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shuffle_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shuffle_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_shuffle_epi32 () { let a = _mm_setr_epi32 (5 , 10 , 15 , 20) ; let r = _mm_shuffle_epi32 :: < 0b00_01_01_11 > (a) ; let e = _mm_setr_epi32 (20 , 10 , 10 , 5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_shufflehi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shufflehi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shufflehi_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_shufflehi_epi16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 10 , 15 , 20) ; let r = _mm_shufflehi_epi16 :: < 0b00_01_01_11 > (a) ; let e = _mm_setr_epi16 (1 , 2 , 3 , 4 , 20 , 10 , 10 , 5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_shufflelo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shufflelo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shufflelo_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_shufflelo_epi16 () { let a = _mm_setr_epi16 (5 , 10 , 15 , 20 , 1 , 2 , 3 , 4) ; let r = _mm_shufflelo_epi16 :: < 0b00_01_01_11 > (a) ; let e = _mm_setr_epi16 (20 , 10 , 10 , 5 , 1 , 2 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpackhi_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpackhi_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ; let r = _mm_unpackhi_epi8 (a , b) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (8 , 24 , 9 , 25 , 10 , 26 , 11 , 27 , 12 , 28 , 13 , 29 , 14 , 30 , 15 , 31 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpackhi_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpackhi_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_unpackhi_epi16 (a , b) ; let e = _mm_setr_epi16 (4 , 12 , 5 , 13 , 6 , 14 , 7 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpackhi_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpackhi_epi32 () { let a = _mm_setr_epi32 (0 , 1 , 2 , 3) ; let b = _mm_setr_epi32 (4 , 5 , 6 , 7) ; let r = _mm_unpackhi_epi32 (a , b) ; let e = _mm_setr_epi32 (2 , 6 , 3 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpackhi_epi64 () { let a = _mm_setr_epi64x (0 , 1) ; let b = _mm_setr_epi64x (2 , 3) ; let r = _mm_unpackhi_epi64 (a , b) ; let e = _mm_setr_epi64x (1 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpacklo_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_epi8_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpacklo_epi8 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,) ; # [rustfmt :: skip] let b = _mm_setr_epi8 (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ; let r = _mm_unpacklo_epi8 (a , b) ; # [rustfmt :: skip] let e = _mm_setr_epi8 (0 , 16 , 1 , 17 , 2 , 18 , 3 , 19 , 4 , 20 , 5 , 21 , 6 , 22 , 7 , 23 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpacklo_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_epi16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpacklo_epi16 () { let a = _mm_setr_epi16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm_setr_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_unpacklo_epi16 (a , b) ; let e = _mm_setr_epi16 (0 , 8 , 1 , 9 , 2 , 10 , 3 , 11) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpacklo_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpacklo_epi32 () { let a = _mm_setr_epi32 (0 , 1 , 2 , 3) ; let b = _mm_setr_epi32 (4 , 5 , 6 , 7) ; let r = _mm_unpacklo_epi32 (a , b) ; let e = _mm_setr_epi32 (0 , 4 , 1 , 5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_epi64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpacklo_epi64 () { let a = _mm_setr_epi64x (0 , 1) ; let b = _mm_setr_epi64x (2 , 3) ; let r = _mm_unpacklo_epi64 (a , b) ; let e = _mm_setr_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_add_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_add_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (6.0 , 2.0)) ; }
}

macro_rules! test_mm_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_add_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_add_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (6.0 , 12.0)) ; }
}

macro_rules! test_mm_div_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_div_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_div_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_div_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_div_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (0.2 , 2.0)) ; }
}

macro_rules! test_mm_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_div_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_div_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_div_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (0.2 , 0.2)) ; }
}

macro_rules! test_mm_max_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_max_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_max_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 2.0)) ; }
}

macro_rules! test_mm_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_max_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_max_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 10.0)) ; let a = _mm_setr_pd (- 0.0 , 0.0) ; let b = _mm_setr_pd (0.0 , 0.0) ; let r1 : [u8 ; 16] = transmute (_mm_max_pd (a , b)) ; let r2 : [u8 ; 16] = transmute (_mm_max_pd (b , a)) ; let a : [u8 ; 16] = transmute (a) ; let b : [u8 ; 16] = transmute (b) ; assert_eq ! (r1 , b) ; assert_eq ! (r2 , a) ; assert_ne ! (a , b) ; }
}

macro_rules! test_mm_min_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_min_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_min_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (1.0 , 2.0)) ; }
}

macro_rules! test_mm_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_min_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_min_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (1.0 , 2.0)) ; let a = _mm_setr_pd (- 0.0 , 0.0) ; let b = _mm_setr_pd (0.0 , 0.0) ; let r1 : [u8 ; 16] = transmute (_mm_min_pd (a , b)) ; let r2 : [u8 ; 16] = transmute (_mm_min_pd (b , a)) ; let a : [u8 ; 16] = transmute (a) ; let b : [u8 ; 16] = transmute (b) ; assert_eq ! (r1 , b) ; assert_eq ! (r2 , a) ; assert_ne ! (a , b) ; }
}

macro_rules! test_mm_mul_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mul_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mul_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mul_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_mul_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 2.0)) ; }
}

macro_rules! test_mm_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mul_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_mul_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_mul_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (5.0 , 20.0)) ; }
}

macro_rules! test_mm_sqrt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sqrt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sqrt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sqrt_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_sqrt_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (5.0f64 . sqrt () , 2.0)) ; }
}

macro_rules! test_mm_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sqrt_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sqrt_pd () { let r = _mm_sqrt_pd (_mm_setr_pd (1.0 , 2.0)) ; assert_eq_m128d (r , _mm_setr_pd (1.0f64 . sqrt () , 2.0f64 . sqrt ())) ; }
}

macro_rules! test_mm_sub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_sd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_sub_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (- 4.0 , 2.0)) ; }
}

macro_rules! test_mm_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_sub_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (5.0 , 10.0) ; let r = _mm_sub_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (- 4.0 , - 8.0)) ; }
}

macro_rules! test_mm_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_and_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_and_pd () { let a = transmute (u64x2 :: splat (5)) ; let b = transmute (u64x2 :: splat (3)) ; let r = _mm_and_pd (a , b) ; let e = transmute (u64x2 :: splat (1)) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_andnot_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_andnot_pd () { let a = transmute (u64x2 :: splat (5)) ; let b = transmute (u64x2 :: splat (3)) ; let r = _mm_andnot_pd (a , b) ; let e = transmute (u64x2 :: splat (2)) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_or_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_or_pd () { let a = transmute (u64x2 :: splat (5)) ; let b = transmute (u64x2 :: splat (3)) ; let r = _mm_or_pd (a , b) ; let e = transmute (u64x2 :: splat (7)) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_xor_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_xor_pd () { let a = transmute (u64x2 :: splat (5)) ; let b = transmute (u64x2 :: splat (3)) ; let r = _mm_xor_pd (a , b) ; let e = transmute (u64x2 :: splat (6)) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_cmpeq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpeq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpeq_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmplt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmplt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmplt_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmple_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmple_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmple_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpgt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpgt_sd () { let (a , b) = (_mm_setr_pd (5.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpgt_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpge_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpge_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpge_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpord_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpord_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpord_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpord_sd () { let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpord_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpunord_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpunord_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpunord_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpunord_sd () { let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpunord_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpneq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpneq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpneq_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnlt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnlt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnlt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnlt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpnlt_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnle_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnle_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnle_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnle_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpnle_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpngt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpngt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpngt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpngt_sd () { let (a , b) = (_mm_setr_pd (5.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpngt_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnge_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnge_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnge_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnge_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 2.0f64 . to_bits () as i64) ; let r = transmute :: < _ , __m128i > (_mm_cmpnge_sd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpeq_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpeq_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpeq_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmplt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmplt_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmplt_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmple_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmple_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmple_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpgt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpgt_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpgt_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpge_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpge_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpge_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpord_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpord_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpord_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpord_pd () { let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpord_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpunord_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpunord_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpunord_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpunord_pd () { let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpunord_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpneq_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpneq_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (! 0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpneq_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnlt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnlt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnlt_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnlt_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (5.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpnlt_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnle_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnle_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpnle_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpngt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpngt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpngt_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpngt_pd () { let (a , b) = (_mm_setr_pd (5.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpngt_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cmpnge_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnge_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnge_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cmpnge_pd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; let e = _mm_setr_epi64x (0 , ! 0) ; let r = transmute :: < _ , __m128i > (_mm_cmpnge_pd (a , b)) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_comieq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comieq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comieq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comieq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comieq_sd (a , b) != 0) ; let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comieq_sd (a , b) == 0) ; }
}

macro_rules! test_mm_comilt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comilt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comilt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comilt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comilt_sd (a , b) == 0) ; }
}

macro_rules! test_mm_comile_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comile_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comile_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comile_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comile_sd (a , b) != 0) ; }
}

macro_rules! test_mm_comigt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comigt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comigt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comigt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comigt_sd (a , b) == 0) ; }
}

macro_rules! test_mm_comige_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comige_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comige_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comige_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comige_sd (a , b) != 0) ; }
}

macro_rules! test_mm_comineq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comineq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comineq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_comineq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_comineq_sd (a , b) == 0) ; }
}

macro_rules! test_mm_ucomieq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomieq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomieq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomieq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomieq_sd (a , b) != 0) ; let (a , b) = (_mm_setr_pd (NAN , 2.0) , _mm_setr_pd (NAN , 3.0)) ; assert ! (_mm_ucomieq_sd (a , b) == 0) ; }
}

macro_rules! test_mm_ucomilt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomilt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomilt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomilt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomilt_sd (a , b) == 0) ; }
}

macro_rules! test_mm_ucomile_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomile_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomile_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomile_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomile_sd (a , b) != 0) ; }
}

macro_rules! test_mm_ucomigt_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomigt_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomigt_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomigt_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomigt_sd (a , b) == 0) ; }
}

macro_rules! test_mm_ucomige_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomige_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomige_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomige_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomige_sd (a , b) != 0) ; }
}

macro_rules! test_mm_ucomineq_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomineq_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomineq_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_ucomineq_sd () { let (a , b) = (_mm_setr_pd (1.0 , 2.0) , _mm_setr_pd (1.0 , 3.0)) ; assert ! (_mm_ucomineq_sd (a , b) == 0) ; }
}

macro_rules! test_mm_movemask_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movemask_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movemask_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_movemask_pd () { let r = _mm_movemask_pd (_mm_setr_pd (- 1.0 , 5.0)) ; assert_eq ! (r , 0b01) ; let r = _mm_movemask_pd (_mm_setr_pd (- 1.0 , - 5.0)) ; assert_eq ! (r , 0b11) ; }
}
mkitem!{mkstruct!{# [repr (align (16))] struct Memory { data : [f64 ; 4] , }}}

macro_rules! test_mm_load_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_load_pd () { let mem = Memory { data : [1.0f64 , 2.0 , 3.0 , 4.0] , } ; let vals = & mem . data ; let d = vals . as_ptr () ; let r = _mm_load_pd (d) ; assert_eq_m128d (r , _mm_setr_pd (1.0 , 2.0)) ; }
}

macro_rules! test_mm_load_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_load_sd () { let a = 1. ; let expected = _mm_setr_pd (a , 0.) ; let r = _mm_load_sd (& a) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_loadh_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadh_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadh_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadh_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = 3. ; let expected = _mm_setr_pd (_mm_cvtsd_f64 (a) , 3.) ; let r = _mm_loadh_pd (a , & b) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_loadl_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadl_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadl_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadl_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = 3. ; let expected = _mm_setr_pd (3. , get_m128d (a , 1)) ; let r = _mm_loadl_pd (a , & b) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_stream_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_pd_introspect!();
    # [simd_test (enable = "sse2")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_pd () { # [repr (align (128))] struct Memory { pub data : [f64 ; 2] , } let a = _mm_set1_pd (7.0) ; let mut mem = Memory { data : [- 1.0 ; 2] } ; _mm_stream_pd (ptr :: addr_of_mut ! (mem . data [0]) , a) ; for i in 0 .. 2 { assert_eq ! (mem . data [i] , get_m128d (a , i)) ; } }
}

macro_rules! test_mm_store_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_store_sd () { let mut dest = 0. ; let a = _mm_setr_pd (1. , 2.) ; _mm_store_sd (& mut dest , a) ; assert_eq ! (dest , _mm_cvtsd_f64 (a)) ; }
}

macro_rules! test_mm_store_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_store_pd () { let mut mem = Memory { data : [0.0f64 ; 4] } ; let vals = & mut mem . data ; let a = _mm_setr_pd (1.0 , 2.0) ; let d = vals . as_mut_ptr () ; _mm_store_pd (d , * black_box (& a)) ; assert_eq ! (vals [0] , 1.0) ; assert_eq ! (vals [1] , 2.0) ; }
}

macro_rules! test_mm_storeu_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeu_pd () { let mut mem = Memory { data : [0.0f64 ; 4] } ; let vals = & mut mem . data ; let a = _mm_setr_pd (1.0 , 2.0) ; let mut ofs = 0 ; let mut p = vals . as_mut_ptr () ; if (p as usize) & 0xf == 0 { ofs = 1 ; p = p . add (1) ; } _mm_storeu_pd (p , * black_box (& a)) ; if ofs > 0 { assert_eq ! (vals [ofs - 1] , 0.0) ; } assert_eq ! (vals [ofs + 0] , 1.0) ; assert_eq ! (vals [ofs + 1] , 2.0) ; }
}

macro_rules! test_mm_storeu_si16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_si16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_si16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeu_si16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let mut r = _mm_setr_epi16 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; _mm_storeu_si16 (ptr :: addr_of_mut ! (r) . cast () , a) ; let e = _mm_setr_epi16 (1 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_storeu_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_si32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeu_si32 () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let mut r = _mm_setr_epi32 (5 , 6 , 7 , 8) ; _mm_storeu_si32 (ptr :: addr_of_mut ! (r) . cast () , a) ; let e = _mm_setr_epi32 (1 , 6 , 7 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_storeu_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_si64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeu_si64 () { let a = _mm_setr_epi64x (1 , 2) ; let mut r = _mm_setr_epi64x (3 , 4) ; _mm_storeu_si64 (ptr :: addr_of_mut ! (r) . cast () , a) ; let e = _mm_setr_epi64x (1 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_store1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store1_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store1_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_store1_pd () { let mut mem = Memory { data : [0.0f64 ; 4] } ; let vals = & mut mem . data ; let a = _mm_setr_pd (1.0 , 2.0) ; let d = vals . as_mut_ptr () ; _mm_store1_pd (d , * black_box (& a)) ; assert_eq ! (vals [0] , 1.0) ; assert_eq ! (vals [1] , 1.0) ; }
}

macro_rules! test_mm_store_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_pd1 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_pd1_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_store_pd1 () { let mut mem = Memory { data : [0.0f64 ; 4] } ; let vals = & mut mem . data ; let a = _mm_setr_pd (1.0 , 2.0) ; let d = vals . as_mut_ptr () ; _mm_store_pd1 (d , * black_box (& a)) ; assert_eq ! (vals [0] , 1.0) ; assert_eq ! (vals [1] , 1.0) ; }
}

macro_rules! test_mm_storer_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storer_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storer_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storer_pd () { let mut mem = Memory { data : [0.0f64 ; 4] } ; let vals = & mut mem . data ; let a = _mm_setr_pd (1.0 , 2.0) ; let d = vals . as_mut_ptr () ; _mm_storer_pd (d , * black_box (& a)) ; assert_eq ! (vals [0] , 2.0) ; assert_eq ! (vals [1] , 1.0) ; }
}

macro_rules! test_mm_storeh_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeh_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeh_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storeh_pd () { let mut dest = 0. ; let a = _mm_setr_pd (1. , 2.) ; _mm_storeh_pd (& mut dest , a) ; assert_eq ! (dest , get_m128d (a , 1)) ; }
}

macro_rules! test_mm_storel_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storel_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storel_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_storel_pd () { let mut dest = 0. ; let a = _mm_setr_pd (1. , 2.) ; _mm_storel_pd (& mut dest , a) ; assert_eq ! (dest , _mm_cvtsd_f64 (a)) ; }
}

macro_rules! test_mm_loadr_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadr_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadr_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadr_pd () { let mut mem = Memory { data : [1.0f64 , 2.0 , 3.0 , 4.0] , } ; let vals = & mut mem . data ; let d = vals . as_ptr () ; let r = _mm_loadr_pd (d) ; assert_eq_m128d (r , _mm_setr_pd (2.0 , 1.0)) ; }
}

macro_rules! test_mm_loadu_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadu_pd () { let mut mem = Memory { data : [1.0f64 , 2.0 , 3.0 , 4.0] , } ; let vals = & mut mem . data ; let mut d = vals . as_ptr () ; let mut offset = 0 ; if (d as usize) & 0xf == 0 { offset = 1 ; d = d . add (offset) ; } let r = _mm_loadu_pd (d) ; let e = _mm_add_pd (_mm_setr_pd (1.0 , 2.0) , _mm_set1_pd (offset as f64)) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_loadu_si16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_si16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_si16_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadu_si16 () { let a = _mm_setr_epi16 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm_loadu_si16 (ptr :: addr_of ! (a) as * const _) ; assert_eq_m128i (r , _mm_setr_epi16 (1 , 0 , 0 , 0 , 0 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_loadu_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_si32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadu_si32 () { let a = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let r = _mm_loadu_si32 (ptr :: addr_of ! (a) as * const _) ; assert_eq_m128i (r , _mm_setr_epi32 (1 , 0 , 0 , 0)) ; }
}

macro_rules! test_mm_loadu_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_si64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_loadu_si64 () { let a = _mm_setr_epi64x (5 , 6) ; let r = _mm_loadu_si64 (ptr :: addr_of ! (a) as * const _) ; assert_eq_m128i (r , _mm_setr_epi64x (5 , 0)) ; }
}

macro_rules! test_mm_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtpd_ps_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtpd_ps () { let r = _mm_cvtpd_ps (_mm_setr_pd (- 1.0 , 5.0)) ; assert_eq_m128 (r , _mm_setr_ps (- 1.0 , 5.0 , 0.0 , 0.0)) ; let r = _mm_cvtpd_ps (_mm_setr_pd (- 1.0 , - 5.0)) ; assert_eq_m128 (r , _mm_setr_ps (- 1.0 , - 5.0 , 0.0 , 0.0)) ; let r = _mm_cvtpd_ps (_mm_setr_pd (f64 :: MAX , f64 :: MIN)) ; assert_eq_m128 (r , _mm_setr_ps (f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0 , 0.0)) ; let r = _mm_cvtpd_ps (_mm_setr_pd (f32 :: MAX as f64 , f32 :: MIN as f64)) ; assert_eq_m128 (r , _mm_setr_ps (f32 :: MAX , f32 :: MIN , 0.0 , 0.0)) ; }
}

macro_rules! test_mm_cvtps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtps_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtps_pd () { let r = _mm_cvtps_pd (_mm_setr_ps (- 1.0 , 2.0 , - 3.0 , 5.0)) ; assert_eq_m128d (r , _mm_setr_pd (- 1.0 , 2.0)) ; let r = _mm_cvtps_pd (_mm_setr_ps (f32 :: MAX , f32 :: INFINITY , f32 :: NEG_INFINITY , f32 :: MIN ,)) ; assert_eq_m128d (r , _mm_setr_pd (f32 :: MAX as f64 , f64 :: INFINITY)) ; }
}

macro_rules! test_mm_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtpd_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtpd_epi32 () { let r = _mm_cvtpd_epi32 (_mm_setr_pd (- 1.0 , 5.0)) ; assert_eq_m128i (r , _mm_setr_epi32 (- 1 , 5 , 0 , 0)) ; let r = _mm_cvtpd_epi32 (_mm_setr_pd (- 1.0 , - 5.0)) ; assert_eq_m128i (r , _mm_setr_epi32 (- 1 , - 5 , 0 , 0)) ; let r = _mm_cvtpd_epi32 (_mm_setr_pd (f64 :: MAX , f64 :: MIN)) ; assert_eq_m128i (r , _mm_setr_epi32 (i32 :: MIN , i32 :: MIN , 0 , 0)) ; let r = _mm_cvtpd_epi32 (_mm_setr_pd (f64 :: INFINITY , f64 :: NEG_INFINITY)) ; assert_eq_m128i (r , _mm_setr_epi32 (i32 :: MIN , i32 :: MIN , 0 , 0)) ; let r = _mm_cvtpd_epi32 (_mm_setr_pd (f64 :: NAN , f64 :: NAN)) ; assert_eq_m128i (r , _mm_setr_epi32 (i32 :: MIN , i32 :: MIN , 0 , 0)) ; }
}

macro_rules! test_mm_cvtsd_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_si32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsd_si32 () { let r = _mm_cvtsd_si32 (_mm_setr_pd (- 2.0 , 5.0)) ; assert_eq ! (r , - 2) ; let r = _mm_cvtsd_si32 (_mm_setr_pd (f64 :: MAX , f64 :: MIN)) ; assert_eq ! (r , i32 :: MIN) ; let r = _mm_cvtsd_si32 (_mm_setr_pd (f64 :: NAN , f64 :: NAN)) ; assert_eq ! (r , i32 :: MIN) ; }
}

macro_rules! test_mm_cvtsd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_ss_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsd_ss () { let a = _mm_setr_ps (- 1.1 , - 2.2 , 3.3 , 4.4) ; let b = _mm_setr_pd (2.0 , - 5.0) ; let r = _mm_cvtsd_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (2.0 , - 2.2 , 3.3 , 4.4)) ; let a = _mm_setr_ps (- 1.1 , f32 :: NEG_INFINITY , f32 :: MAX , f32 :: NEG_INFINITY) ; let b = _mm_setr_pd (f64 :: INFINITY , - 5.0) ; let r = _mm_cvtsd_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (f32 :: INFINITY , f32 :: NEG_INFINITY , f32 :: MAX , f32 :: NEG_INFINITY ,) ,) ; }
}

macro_rules! test_mm_cvtsd_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_f64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_f64_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtsd_f64 () { let r = _mm_cvtsd_f64 (_mm_setr_pd (- 1.1 , 2.2)) ; assert_eq ! (r , - 1.1) ; }
}

macro_rules! test_mm_cvtss_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvtss_sd () { let a = _mm_setr_pd (- 1.1 , 2.2) ; let b = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let r = _mm_cvtss_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (1.0 , 2.2)) ; let a = _mm_setr_pd (- 1.1 , f64 :: INFINITY) ; let b = _mm_setr_ps (f32 :: NEG_INFINITY , 2.0 , 3.0 , 4.0) ; let r = _mm_cvtss_sd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (f64 :: NEG_INFINITY , f64 :: INFINITY)) ; }
}

macro_rules! test_mm_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttpd_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvttpd_epi32 () { let a = _mm_setr_pd (- 1.1 , 2.2) ; let r = _mm_cvttpd_epi32 (a) ; assert_eq_m128i (r , _mm_setr_epi32 (- 1 , 2 , 0 , 0)) ; let a = _mm_setr_pd (f64 :: NEG_INFINITY , f64 :: NAN) ; let r = _mm_cvttpd_epi32 (a) ; assert_eq_m128i (r , _mm_setr_epi32 (i32 :: MIN , i32 :: MIN , 0 , 0)) ; }
}

macro_rules! test_mm_cvttsd_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttsd_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttsd_si32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvttsd_si32 () { let a = _mm_setr_pd (- 1.1 , 2.2) ; let r = _mm_cvttsd_si32 (a) ; assert_eq ! (r , - 1) ; let a = _mm_setr_pd (f64 :: NEG_INFINITY , f64 :: NAN) ; let r = _mm_cvttsd_si32 (a) ; assert_eq ! (r , i32 :: MIN) ; }
}

macro_rules! test_mm_cvttps_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttps_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttps_epi32_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_cvttps_epi32 () { let a = _mm_setr_ps (- 1.1 , 2.2 , - 3.3 , 6.6) ; let r = _mm_cvttps_epi32 (a) ; assert_eq_m128i (r , _mm_setr_epi32 (- 1 , 2 , - 3 , 6)) ; let a = _mm_setr_ps (f32 :: NEG_INFINITY , f32 :: INFINITY , f32 :: MIN , f32 :: MAX) ; let r = _mm_cvttps_epi32 (a) ; assert_eq_m128i (r , _mm_setr_epi32 (i32 :: MIN , i32 :: MIN , i32 :: MIN , i32 :: MIN)) ; }
}

macro_rules! test_mm_set_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_sd () { let r = _mm_set_sd (- 1.0_f64) ; assert_eq_m128d (r , _mm_setr_pd (- 1.0_f64 , 0_f64)) ; }
}

macro_rules! test_mm_set1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set1_pd () { let r = _mm_set1_pd (- 1.0_f64) ; assert_eq_m128d (r , _mm_setr_pd (- 1.0_f64 , - 1.0_f64)) ; }
}

macro_rules! test_mm_set_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_pd1 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_pd1_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_pd1 () { let r = _mm_set_pd1 (- 2.0_f64) ; assert_eq_m128d (r , _mm_setr_pd (- 2.0_f64 , - 2.0_f64)) ; }
}

macro_rules! test_mm_set_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_set_pd () { let r = _mm_set_pd (1.0_f64 , 5.0_f64) ; assert_eq_m128d (r , _mm_setr_pd (5.0_f64 , 1.0_f64)) ; }
}

macro_rules! test_mm_setr_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setr_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setr_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setr_pd () { let r = _mm_setr_pd (1.0_f64 , - 5.0_f64) ; assert_eq_m128d (r , _mm_setr_pd (1.0_f64 , - 5.0_f64)) ; }
}

macro_rules! test_mm_setzero_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setzero_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setzero_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_setzero_pd () { let r = _mm_setzero_pd () ; assert_eq_m128d (r , _mm_setr_pd (0_f64 , 0_f64)) ; }
}

macro_rules! test_mm_load1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load1_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load1_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_load1_pd () { let d = - 5.0 ; let r = _mm_load1_pd (& d) ; assert_eq_m128d (r , _mm_setr_pd (d , d)) ; }
}

macro_rules! test_mm_load_pd1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_pd1 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_pd1_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_load_pd1 () { let d = - 5.0 ; let r = _mm_load_pd1 (& d) ; assert_eq_m128d (r , _mm_setr_pd (d , d)) ; }
}

macro_rules! test_mm_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpackhi_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (3.0 , 4.0) ; let r = _mm_unpackhi_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (2.0 , 4.0)) ; }
}

macro_rules! test_mm_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_unpacklo_pd () { let a = _mm_setr_pd (1.0 , 2.0) ; let b = _mm_setr_pd (3.0 , 4.0) ; let r = _mm_unpacklo_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (1.0 , 3.0)) ; }
}

macro_rules! test_mm_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shuffle_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_shuffle_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (3. , 4.) ; let expected = _mm_setr_pd (1. , 3.) ; let r = _mm_shuffle_pd :: < 0b00_00_00_00 > (a , b) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_move_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_move_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_move_sd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_move_sd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (3. , 4.) ; let expected = _mm_setr_pd (3. , 2.) ; let r = _mm_move_sd (a , b) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_castpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castpd_ps_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castpd_ps () { let a = _mm_set1_pd (0.) ; let expected = _mm_set1_ps (0.) ; let r = _mm_castpd_ps (a) ; assert_eq_m128 (r , expected) ; }
}

macro_rules! test_mm_castpd_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castpd_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castpd_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castpd_si128 () { let a = _mm_set1_pd (0.) ; let expected = _mm_set1_epi64x (0) ; let r = _mm_castpd_si128 (a) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_castps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castps_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castps_pd () { let a = _mm_set1_ps (0.) ; let expected = _mm_set1_pd (0.) ; let r = _mm_castps_pd (a) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_castps_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castps_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castps_si128_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castps_si128 () { let a = _mm_set1_ps (0.) ; let expected = _mm_set1_epi32 (0) ; let r = _mm_castps_si128 (a) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_castsi128_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castsi128_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castsi128_pd_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castsi128_pd () { let a = _mm_set1_epi64x (0) ; let expected = _mm_set1_pd (0.) ; let r = _mm_castsi128_pd (a) ; assert_eq_m128d (r , expected) ; }
}

macro_rules! test_mm_castsi128_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_castsi128_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_castsi128_ps_introspect!();
    # [simd_test (enable = "sse2")] unsafe fn test_mm_castsi128_ps () { let a = _mm_set1_epi32 (0) ; let expected = _mm_set1_ps (0.) ; let r = _mm_castsi128_ps (a) ; assert_eq_m128 (r , expected) ; }
} 
            }}