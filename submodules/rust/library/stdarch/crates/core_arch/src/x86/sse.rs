mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , intrinsics :: simd :: * , intrinsics :: sqrtf32 , mem , ptr , } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_add_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_ss_introspect!();
    # [doc = " Adds the first component of `a` and `b`, the other components are copied"] # [doc = " from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (addss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , _mm_cvtss_f32 (a) + _mm_cvtss_f32 (b)) } }
}

macro_rules! _mm_add_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_add_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_add_ps_introspect!();
    # [doc = " Adds packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (addps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_add_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_add (a , b) } }
}

macro_rules! _mm_sub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_ss_introspect!();
    # [doc = " Subtracts the first component of `b` from `a`, the other components are"] # [doc = " copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (subss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , _mm_cvtss_f32 (a) - _mm_cvtss_f32 (b)) } }
}

macro_rules! _mm_sub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_sub_ps_introspect!();
    # [doc = " Subtracts packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sub_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (subps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sub_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_sub (a , b) } }
}

macro_rules! _mm_mul_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mul_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_mul_ss_introspect!();
    # [doc = " Multiplies the first component of `a` and `b`, the other components are"] # [doc = " copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mul_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (mulss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mul_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , _mm_cvtss_f32 (a) * _mm_cvtss_f32 (b)) } }
}

macro_rules! _mm_mul_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mul_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mul_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mul_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (mulps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_mul_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_mul (a , b) } }
}

macro_rules! _mm_div_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_div_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_div_ss_introspect!();
    # [doc = " Divides the first component of `b` by `a`, the other components are"] # [doc = " copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_div_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (divss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_div_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , _mm_cvtss_f32 (a) / _mm_cvtss_f32 (b)) } }
}

macro_rules! _mm_div_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_div_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_div_ps_introspect!();
    # [doc = " Divides packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_div_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (divps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_div_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_div (a , b) } }
}

macro_rules! _mm_sqrt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sqrt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_sqrt_ss_introspect!();
    # [doc = " Returns the square root of the first single-precision (32-bit)"] # [doc = " floating-point element in `a`, the other elements are unchanged."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sqrt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (sqrtss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sqrt_ss (a : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , sqrtf32 (_mm_cvtss_f32 (a))) } }
}

macro_rules! _mm_sqrt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sqrt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_sqrt_ps_introspect!();
    # [doc = " Returns the square root of packed single-precision (32-bit) floating-point"] # [doc = " elements in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sqrt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (sqrtps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sqrt_ps (a : __m128) -> __m128 { unsafe { simd_fsqrt (a) } }
}

macro_rules! _mm_rcp_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_rcp_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_rcp_ss_introspect!();
    # [doc = " Returns the approximate reciprocal of the first single-precision"] # [doc = " (32-bit) floating-point element in `a`, the other elements are unchanged."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rcp_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (rcpss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_rcp_ss (a : __m128) -> __m128 { unsafe { rcpss (a) } }
}

macro_rules! _mm_rcp_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_rcp_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_rcp_ps_introspect!();
    # [doc = " Returns the approximate reciprocal of packed single-precision (32-bit)"] # [doc = " floating-point elements in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rcp_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (rcpps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_rcp_ps (a : __m128) -> __m128 { unsafe { rcpps (a) } }
}

macro_rules! _mm_rsqrt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_rsqrt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_rsqrt_ss_introspect!();
    # [doc = " Returns the approximate reciprocal square root of the first single-precision"] # [doc = " (32-bit) floating-point element in `a`, the other elements are unchanged."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (rsqrtss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_rsqrt_ss (a : __m128) -> __m128 { unsafe { rsqrtss (a) } }
}

macro_rules! _mm_rsqrt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_rsqrt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_rsqrt_ps_introspect!();
    # [doc = " Returns the approximate reciprocal square root of packed single-precision"] # [doc = " (32-bit) floating-point elements in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (rsqrtps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_rsqrt_ps (a : __m128) -> __m128 { unsafe { rsqrtps (a) } }
}

macro_rules! _mm_min_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_ss_introspect!();
    # [doc = " Compares the first single-precision (32-bit) floating-point element of `a`"] # [doc = " and `b`, and return the minimum value in the first element of the return"] # [doc = " value, the other elements are copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (minss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_ss (a : __m128 , b : __m128) -> __m128 { unsafe { minss (a , b) } }
}

macro_rules! _mm_min_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_min_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_min_ps_introspect!();
    # [doc = " Compares packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`, and return the corresponding minimum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_min_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (minps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_min_ps (a : __m128 , b : __m128) -> __m128 { unsafe { minps (a , b) } }
}

macro_rules! _mm_max_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_ss_introspect!();
    # [doc = " Compares the first single-precision (32-bit) floating-point element of `a`"] # [doc = " and `b`, and return the maximum value in the first element of the return"] # [doc = " value, the other elements are copied from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (maxss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_ss (a : __m128 , b : __m128) -> __m128 { unsafe { maxss (a , b) } }
}

macro_rules! _mm_max_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_max_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_max_ps_introspect!();
    # [doc = " Compares packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b`, and return the corresponding maximum values."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_max_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (maxps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_max_ps (a : __m128 , b : __m128) -> __m128 { unsafe { maxps (a , b) } }
}

macro_rules! _mm_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_and_ps_introspect!();
    # [doc = " Bitwise AND of packed single-precision (32-bit) floating-point elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_and_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , any (target_arch = "x86_64" , target_feature = "sse2")) , assert_instr (andps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_and_ps (a : __m128 , b : __m128) -> __m128 { unsafe { let a : __m128i = mem :: transmute (a) ; let b : __m128i = mem :: transmute (b) ; mem :: transmute (simd_and (a , b)) } }
}

macro_rules! _mm_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_andnot_ps_introspect!();
    # [doc = " Bitwise AND-NOT of packed single-precision (32-bit) floating-point"] # [doc = " elements."] # [doc = ""] # [doc = " Computes `!a & b` for each bit in `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_andnot_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , any (target_arch = "x86_64" , target_feature = "sse2")) , assert_instr (andnps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_andnot_ps (a : __m128 , b : __m128) -> __m128 { unsafe { let a : __m128i = mem :: transmute (a) ; let b : __m128i = mem :: transmute (b) ; let mask : __m128i = mem :: transmute (i32x4 :: splat (- 1)) ; mem :: transmute (simd_and (simd_xor (mask , a) , b)) } }
}

macro_rules! _mm_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_or_ps_introspect!();
    # [doc = " Bitwise OR of packed single-precision (32-bit) floating-point elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_or_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , any (target_arch = "x86_64" , target_feature = "sse2")) , assert_instr (orps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_or_ps (a : __m128 , b : __m128) -> __m128 { unsafe { let a : __m128i = mem :: transmute (a) ; let b : __m128i = mem :: transmute (b) ; mem :: transmute (simd_or (a , b)) } }
}

macro_rules! _mm_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_xor_ps_introspect!();
    # [doc = " Bitwise exclusive OR of packed single-precision (32-bit) floating-point"] # [doc = " elements."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_xor_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , any (target_arch = "x86_64" , target_feature = "sse2")) , assert_instr (xorps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_xor_ps (a : __m128 , b : __m128) -> __m128 { unsafe { let a : __m128i = mem :: transmute (a) ; let b : __m128i = mem :: transmute (b) ; mem :: transmute (simd_xor (a , b)) } }
}

macro_rules! _mm_cmpeq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for equality. The lowest 32 bits of"] # [doc = " the result will be `0xffffffff` if the two inputs are equal, or `0`"] # [doc = " otherwise. The upper 96 bits of the result are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpeqss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 0) } }
}

macro_rules! _mm_cmplt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for less than. The lowest 32 bits"] # [doc = " of the result will be `0xffffffff` if `a.extract(0)` is less than"] # [doc = " `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the"] # [doc = " upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpltss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 1) } }
}

macro_rules! _mm_cmple_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmple_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmple_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for less than or equal. The lowest"] # [doc = " 32 bits of the result will be `0xffffffff` if `a.extract(0)` is less than"] # [doc = " or equal `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result"] # [doc = " are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmple_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpless))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmple_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 2) } }
}

macro_rules! _mm_cmpgt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for greater than. The lowest 32"] # [doc = " bits of the result will be `0xffffffff` if `a.extract(0)` is greater"] # [doc = " than `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result"] # [doc = " are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpltss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , cmpss (b , a , 1) , [4 , 1 , 2 , 3]) } }
}

macro_rules! _mm_cmpge_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpge_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpge_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for greater than or equal. The"] # [doc = " lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is"] # [doc = " greater than or equal `b.extract(0)`, or `0` otherwise. The upper 96 bits"] # [doc = " of the result are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpge_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpless))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpge_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , cmpss (b , a , 2) , [4 , 1 , 2 , 3]) } }
}

macro_rules! _mm_cmpneq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpneq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpneq_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for inequality. The lowest 32 bits"] # [doc = " of the result will be `0xffffffff` if `a.extract(0)` is not equal to"] # [doc = " `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the"] # [doc = " upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpneq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpneqss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpneq_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 4) } }
}

macro_rules! _mm_cmpnlt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnlt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnlt_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for not-less-than. The lowest 32"] # [doc = " bits of the result will be `0xffffffff` if `a.extract(0)` is not less than"] # [doc = " `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the"] # [doc = " upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnlt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnltss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnlt_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 5) } }
}

macro_rules! _mm_cmpnle_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnle_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnle_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for not-less-than-or-equal. The"] # [doc = " lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not"] # [doc = " less than or equal to `b.extract(0)`, or `0` otherwise. The upper 96 bits"] # [doc = " of the result are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnle_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnless))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnle_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 6) } }
}

macro_rules! _mm_cmpngt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpngt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpngt_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for not-greater-than. The lowest 32"] # [doc = " bits of the result will be `0xffffffff` if `a.extract(0)` is not greater"] # [doc = " than `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are"] # [doc = " the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpngt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnltss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpngt_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , cmpss (b , a , 5) , [4 , 1 , 2 , 3]) } }
}

macro_rules! _mm_cmpnge_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnge_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnge_ss_introspect!();
    # [doc = " Compares the lowest `f32` of both inputs for not-greater-than-or-equal. The"] # [doc = " lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not"] # [doc = " greater than or equal to `b.extract(0)`, or `0` otherwise. The upper 96"] # [doc = " bits of the result are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnge_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnless))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnge_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , cmpss (b , a , 6) , [4 , 1 , 2 , 3]) } }
}

macro_rules! _mm_cmpord_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpord_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpord_ss_introspect!();
    # [doc = " Checks if the lowest `f32` of both inputs are ordered. The lowest 32 bits of"] # [doc = " the result will be `0xffffffff` if neither of `a.extract(0)` or"] # [doc = " `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result"] # [doc = " are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpord_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpordss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpord_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 7) } }
}

macro_rules! _mm_cmpunord_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpunord_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpunord_ss_introspect!();
    # [doc = " Checks if the lowest `f32` of both inputs are unordered. The lowest 32 bits"] # [doc = " of the result will be `0xffffffff` if any of `a.extract(0)` or"] # [doc = " `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result"] # [doc = " are the upper 96 bits of `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpunord_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpunordss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpunord_ss (a : __m128 , b : __m128) -> __m128 { unsafe { cmpss (a , b , 3) } }
}

macro_rules! _mm_cmpeq_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpeq_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpeq_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input elements"] # [doc = " were equal, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpeq_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpeqps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpeq_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 0) } }
}

macro_rules! _mm_cmplt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmplt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmplt_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is less than the corresponding element in `b`, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmplt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpltps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmplt_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 1) } }
}

macro_rules! _mm_cmple_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmple_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmple_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is less than or equal to the corresponding element in `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmple_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpleps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmple_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 2) } }
}

macro_rules! _mm_cmpgt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpgt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpgt_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is greater than the corresponding element in `b`, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpgt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpltps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpgt_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 1) } }
}

macro_rules! _mm_cmpge_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpge_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpge_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is greater than or equal to the corresponding element in `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpge_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpleps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpge_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 2) } }
}

macro_rules! _mm_cmpneq_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpneq_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpneq_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input elements"] # [doc = " are **not** equal, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpneq_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpneqps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpneq_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 4) } }
}

macro_rules! _mm_cmpnlt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnlt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnlt_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is **not** less than the corresponding element in `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnlt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnltps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnlt_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 5) } }
}

macro_rules! _mm_cmpnle_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnle_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnle_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is **not** less than or equal to the corresponding element in `b`, or"] # [doc = " `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnle_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnleps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnle_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (a , b , 6) } }
}

macro_rules! _mm_cmpngt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpngt_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpngt_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is **not** greater than the corresponding element in `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpngt_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnltps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpngt_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 5) } }
}

macro_rules! _mm_cmpnge_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpnge_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpnge_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " The result in the output vector will be `0xffffffff` if the input element"] # [doc = " in `a` is **not** greater than or equal to the corresponding element in `b`,"] # [doc = " or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpnge_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpnleps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpnge_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 6) } }
}

macro_rules! _mm_cmpord_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpord_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpord_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " Returns four floats that have one of two possible bit patterns. The element"] # [doc = " in the output vector will be `0xffffffff` if the input elements in `a` and"] # [doc = " `b` are ordered (i.e., neither of them is a NaN), or 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpord_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpordps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpord_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 7) } }
}

macro_rules! _mm_cmpunord_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cmpunord_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cmpunord_ps_introspect!();
    # [doc = " Compares each of the four floats in `a` to the corresponding element in `b`."] # [doc = " Returns four floats that have one of two possible bit patterns. The element"] # [doc = " in the output vector will be `0xffffffff` if the input elements in `a` and"] # [doc = " `b` are unordered (i.e., at least on of them is a NaN), or 0 otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cmpunord_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cmpunordps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cmpunord_ps (a : __m128 , b : __m128) -> __m128 { unsafe { cmpps (b , a , 3) } }
}

macro_rules! _mm_comieq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comieq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comieq_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if they are equal, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comieq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comieq_ss (a : __m128 , b : __m128) -> i32 { unsafe { comieq_ss (a , b) } }
}

macro_rules! _mm_comilt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comilt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comilt_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is less than the one from `b`, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comilt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comilt_ss (a : __m128 , b : __m128) -> i32 { unsafe { comilt_ss (a , b) } }
}

macro_rules! _mm_comile_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comile_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comile_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is less than or equal to the one from `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comile_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comile_ss (a : __m128 , b : __m128) -> i32 { unsafe { comile_ss (a , b) } }
}

macro_rules! _mm_comigt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comigt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comigt_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is greater than the one from `b`, or `0`"] # [doc = " otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comigt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comigt_ss (a : __m128 , b : __m128) -> i32 { unsafe { comigt_ss (a , b) } }
}

macro_rules! _mm_comige_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comige_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comige_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is greater than or equal to the one from `b`, or"] # [doc = " `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comige_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comige_ss (a : __m128 , b : __m128) -> i32 { unsafe { comige_ss (a , b) } }
}

macro_rules! _mm_comineq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_comineq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_comineq_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if they are **not** equal, or `0` otherwise."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_comineq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (comiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_comineq_ss (a : __m128 , b : __m128) -> i32 { unsafe { comineq_ss (a , b) } }
}

macro_rules! _mm_ucomieq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomieq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomieq_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if they are equal, or `0` otherwise. This instruction will not signal"] # [doc = " an exception if either argument is a quiet NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomieq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomieq_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomieq_ss (a , b) } }
}

macro_rules! _mm_ucomilt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomilt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomilt_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is less than the one from `b`, or `0` otherwise."] # [doc = " This instruction will not signal an exception if either argument is a quiet"] # [doc = " NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomilt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomilt_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomilt_ss (a , b) } }
}

macro_rules! _mm_ucomile_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomile_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomile_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is less than or equal to the one from `b`, or `0`"] # [doc = " otherwise. This instruction will not signal an exception if either argument"] # [doc = " is a quiet NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomile_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomile_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomile_ss (a , b) } }
}

macro_rules! _mm_ucomigt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomigt_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomigt_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is greater than the one from `b`, or `0`"] # [doc = " otherwise. This instruction will not signal an exception if either argument"] # [doc = " is a quiet NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomigt_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomigt_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomigt_ss (a , b) } }
}

macro_rules! _mm_ucomige_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomige_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomige_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if the value from `a` is greater than or equal to the one from `b`, or"] # [doc = " `0` otherwise. This instruction will not signal an exception if either"] # [doc = " argument is a quiet NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomige_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomige_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomige_ss (a , b) } }
}

macro_rules! _mm_ucomineq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_ucomineq_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_ucomineq_ss_introspect!();
    # [doc = " Compares two 32-bit floats from the low-order bits of `a` and `b`. Returns"] # [doc = " `1` if they are **not** equal, or `0` otherwise. This instruction will not"] # [doc = " signal an exception if either argument is a quiet NaN."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_ucomineq_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ucomiss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_ucomineq_ss (a : __m128 , b : __m128) -> i32 { unsafe { ucomineq_ss (a , b) } }
}

macro_rules! _mm_cvtss_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_si32_introspect!();
    # [doc = " Converts the lowest 32 bit float in the input vector to a 32 bit integer."] # [doc = ""] # [doc = " The result is rounded according to the current rounding mode. If the result"] # [doc = " cannot be represented as a 32 bit integer the result will be `0x8000_0000`"] # [doc = " (`i32::MIN`)."] # [doc = ""] # [doc = " This corresponds to the `CVTSS2SI` instruction (with 32 bit output)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_si32)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtss_si32 (a : __m128) -> i32 { unsafe { cvtss2si (a) } }
}

macro_rules! _mm_cvt_ss2si_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_ss2si in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_ss2si_introspect!();
    # [doc = " Alias for [`_mm_cvtss_si32`](fn._mm_cvtss_si32.html)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_ss2si)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvt_ss2si (a : __m128) -> i32 { _mm_cvtss_si32 (a) }
}

macro_rules! _mm_cvttss_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttss_si32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttss_si32_introspect!();
    # [doc = " Converts the lowest 32 bit float in the input vector to a 32 bit integer"] # [doc = " with"] # [doc = " truncation."] # [doc = ""] # [doc = " The result is rounded always using truncation (round towards zero). If the"] # [doc = " result cannot be represented as a 32 bit integer the result will be"] # [doc = " `0x8000_0000` (`i32::MIN`)."] # [doc = ""] # [doc = " This corresponds to the `CVTTSS2SI` instruction (with 32 bit output)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttss_si32)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvttss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvttss_si32 (a : __m128) -> i32 { unsafe { cvttss2si (a) } }
}

macro_rules! _mm_cvtt_ss2si_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_ss2si in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_ss2si_introspect!();
    # [doc = " Alias for [`_mm_cvttss_si32`](fn._mm_cvttss_si32.html)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_ss2si)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvttss2si))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtt_ss2si (a : __m128) -> i32 { _mm_cvttss_si32 (a) }
}

macro_rules! _mm_cvtss_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_f32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_f32_introspect!();
    # [doc = " Extracts the lowest 32 bit float from the input vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_f32)"] # [inline] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtss_f32 (a : __m128) -> f32 { unsafe { simd_extract ! (a , 0) } }
}

macro_rules! _mm_cvtsi32_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsi32_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsi32_ss_introspect!();
    # [doc = " Converts a 32 bit integer to a 32 bit float. The result vector is the input"] # [doc = " vector `a` with the lowest 32 bit float replaced by the converted integer."] # [doc = ""] # [doc = " This intrinsic corresponds to the `CVTSI2SS` instruction (with 32 bit"] # [doc = " input)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsi32_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtsi2ss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvtsi32_ss (a : __m128 , b : i32) -> __m128 { unsafe { cvtsi2ss (a , b) } }
}

macro_rules! _mm_cvt_si2ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_si2ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_si2ss_introspect!();
    # [doc = " Alias for [`_mm_cvtsi32_ss`](fn._mm_cvtsi32_ss.html)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_si2ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (cvtsi2ss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_cvt_si2ss (a : __m128 , b : i32) -> __m128 { _mm_cvtsi32_ss (a , b) }
}

macro_rules! _mm_set_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_ss_introspect!();
    # [doc = " Construct a `__m128` with the lowest element set to `a` and the rest set to"] # [doc = " zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_ss (a : f32) -> __m128 { __m128 ([a , 0.0 , 0.0 , 0.0]) }
}

macro_rules! _mm_set1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set1_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_set1_ps_introspect!();
    # [doc = " Construct a `__m128` with all element set to `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set1_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (shufps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set1_ps (a : f32) -> __m128 { __m128 ([a , a , a , a]) }
}

macro_rules! _mm_set_ps1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_ps1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_ps1_introspect!();
    # [doc = " Alias for [`_mm_set1_ps`](fn._mm_set1_ps.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_ps1)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (shufps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_ps1 (a : f32) -> __m128 { _mm_set1_ps (a) }
}

macro_rules! _mm_set_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_set_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_set_ps_introspect!();
    # [doc = " Construct a `__m128` from four floating point values highest to lowest."] # [doc = ""] # [doc = " Note that `a` will be the highest 32 bits of the result, and `d` the"] # [doc = " lowest. This matches the standard way of writing bit patterns on x86:"] # [doc = ""] # [doc = " ```text"] # [doc = "  bit    127 .. 96  95 .. 64  63 .. 32  31 .. 0"] # [doc = "        +---------+---------+---------+---------+"] # [doc = "        |    a    |    b    |    c    |    d    |   result"] # [doc = "        +---------+---------+---------+---------+"] # [doc = " ```"] # [doc = ""] # [doc = " Alternatively:"] # [doc = ""] # [doc = " ```text"] # [doc = " let v = _mm_set_ps(d, c, b, a);"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_set_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (unpcklps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_set_ps (a : f32 , b : f32 , c : f32 , d : f32) -> __m128 { __m128 ([d , c , b , a]) }
}

macro_rules! _mm_setr_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setr_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_setr_ps_introspect!();
    # [doc = " Construct a `__m128` from four floating point values lowest to highest."] # [doc = ""] # [doc = " This matches the memory order of `__m128`, i.e., `a` will be the lowest 32"] # [doc = " bits of the result, and `d` the highest."] # [doc = ""] # [doc = " ```text"] # [doc = " assert_eq!(__m128::new(a, b, c, d), _mm_setr_ps(a, b, c, d));"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setr_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , any (target_env = "msvc" , target_arch = "x86_64")) , assert_instr (unpcklps))] # [cfg_attr (all (test , all (not (target_env = "msvc") , target_arch = "x86")) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setr_ps (a : f32 , b : f32 , c : f32 , d : f32) -> __m128 { __m128 ([a , b , c , d]) }
}

macro_rules! _mm_setzero_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setzero_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_setzero_ps_introspect!();
    # [doc = " Construct a `__m128` with all elements initialized to zero."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setzero_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (xorps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_setzero_ps () -> __m128 { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _MM_SHUFFLE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_SHUFFLE in module {}", module_path!());
    };
}

mkfn!{
    _MM_SHUFFLE_introspect!();
    # [doc = " A utility function for creating masks to use with Intel shuffle and"] # [doc = " permute intrinsics."] # [inline] # [allow (non_snake_case)] # [unstable (feature = "stdarch_x86_mm_shuffle" , issue = "111147")] pub const fn _MM_SHUFFLE (z : u32 , y : u32 , x : u32 , w : u32) -> i32 { ((z << 6) | (y << 4) | (x << 2) | w) as i32 }
}

macro_rules! _mm_shuffle_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_shuffle_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_shuffle_ps_introspect!();
    # [doc = " Shuffles packed single-precision (32-bit) floating-point elements in `a` and"] # [doc = " `b` using `MASK`."] # [doc = ""] # [doc = " The lower half of result takes values from `a` and the higher half from"] # [doc = " `b`. Mask is split to 2 control bits each to index the element from inputs."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shuffle_ps)"] # [doc = ""] # [doc = " Note that there appears to be a mistake within Intel's Intrinsics Guide."] # [doc = " `_mm_shuffle_ps` is supposed to take an `i32` instead of a `u32`"] # [doc = " as is the case for [other shuffle intrinsics](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_shuffle_)."] # [doc = " Performing an implicit type conversion between an unsigned integer and a signed integer"] # [doc = " does not cause a problem in C, however Rust's commitment to strong typing does not allow this."] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (shufps , MASK = 3))] # [rustc_legacy_const_generics (2)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_shuffle_ps < const MASK : i32 > (a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (MASK , 8) ; unsafe { simd_shuffle ! (a , b , [MASK as u32 & 0b11 , (MASK as u32 >> 2) & 0b11 , ((MASK as u32 >> 4) & 0b11) + 4 , ((MASK as u32 >> 6) & 0b11) + 4 ,] ,) } }
}

macro_rules! _mm_unpackhi_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpackhi_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpackhi_ps_introspect!();
    # [doc = " Unpacks and interleave single-precision (32-bit) floating-point elements"] # [doc = " from the higher half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpackhi_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (unpckhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpackhi_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , b , [2 , 6 , 3 , 7]) } }
}

macro_rules! _mm_unpacklo_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_unpacklo_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_unpacklo_ps_introspect!();
    # [doc = " Unpacks and interleave single-precision (32-bit) floating-point elements"] # [doc = " from the lower half of `a` and `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_unpacklo_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (unpcklps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_unpacklo_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , b , [0 , 4 , 1 , 5]) } }
}

macro_rules! _mm_movehl_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movehl_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_movehl_ps_introspect!();
    # [doc = " Combine higher half of `a` and `b`. The higher half of `b` occupies the"] # [doc = " lower half of result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movehl_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movhlps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movehl_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , b , [6 , 7 , 2 , 3]) } }
}

macro_rules! _mm_movelh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movelh_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_movelh_ps_introspect!();
    # [doc = " Combine lower half of `a` and `b`. The lower half of `b` occupies the"] # [doc = " higher half of result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movelh_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movlhps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movelh_ps (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , b , [0 , 1 , 4 , 5]) } }
}

macro_rules! _mm_movemask_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movemask_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_movemask_ps_introspect!();
    # [doc = " Returns a mask of the most significant bit of each element in `a`."] # [doc = ""] # [doc = " The mask is stored in the 4 least significant bits of the return value."] # [doc = " All other bits are set to `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movemask_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movmskps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movemask_ps (a : __m128) -> i32 { unsafe { let mask : i32x4 = simd_lt (transmute (a) , i32x4 :: ZERO) ; simd_bitmask :: < i32x4 , u8 > (mask) . into () } }
}

macro_rules! _mm_load_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_ss_introspect!();
    # [doc = " Construct a `__m128` with the lowest element read from `p` and the other"] # [doc = " elements set to zero."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVSS` / `MOVSS`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load_ss (p : * const f32) -> __m128 { __m128 ([* p , 0.0 , 0.0 , 0.0]) }
}

macro_rules! _mm_load1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load1_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_load1_ps_introspect!();
    # [doc = " Construct a `__m128` by duplicating the value read from `p` into all"] # [doc = " elements."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVSS` / `MOVSS` followed by some"] # [doc = " shuffling."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load1_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load1_ps (p : * const f32) -> __m128 { let a = * p ; __m128 ([a , a , a , a]) }
}

macro_rules! _mm_load_ps1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_ps1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_ps1_introspect!();
    # [doc = " Alias for [`_mm_load1_ps`](fn._mm_load1_ps.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_ps1)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_load_ps1 (p : * const f32) -> __m128 { _mm_load1_ps (p) }
}

macro_rules! _mm_load_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_load_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_load_ps_introspect!();
    # [doc = " Loads four `f32` values from *aligned* memory into a `__m128`. If the"] # [doc = " pointer is not aligned to a 128-bit boundary (16 bytes) a general"] # [doc = " protection fault will be triggered (fatal program crash)."] # [doc = ""] # [doc = " Use [`_mm_loadu_ps`](fn._mm_loadu_ps.html) for potentially unaligned"] # [doc = " memory."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVAPS` / `MOVAPS`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_load_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_load_ps (p : * const f32) -> __m128 { * (p as * const __m128) }
}

macro_rules! _mm_loadu_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadu_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadu_ps_introspect!();
    # [doc = " Loads four `f32` values from memory into a `__m128`. There are no"] # [doc = " restrictions"] # [doc = " on memory alignment. For aligned memory"] # [doc = " [`_mm_load_ps`](fn._mm_load_ps.html)"] # [doc = " may be faster."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVUPS` / `MOVUPS`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadu_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadu_ps (p : * const f32) -> __m128 { let mut dst = _mm_undefined_ps () ; ptr :: copy_nonoverlapping (p as * const u8 , ptr :: addr_of_mut ! (dst) as * mut u8 , mem :: size_of :: < __m128 > () ,) ; dst }
}

macro_rules! _mm_loadr_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loadr_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_loadr_ps_introspect!();
    # [doc = " Loads four `f32` values from aligned memory into a `__m128` in reverse"] # [doc = " order."] # [doc = ""] # [doc = " If the pointer is not aligned to a 128-bit boundary (16 bytes) a general"] # [doc = " protection fault will be triggered (fatal program crash)."] # [doc = ""] # [doc = " Functionally equivalent to the following code sequence (assuming `p`"] # [doc = " satisfies the alignment restrictions):"] # [doc = ""] # [doc = " ```text"] # [doc = " let a0 = *p;"] # [doc = " let a1 = *p.add(1);"] # [doc = " let a2 = *p.add(2);"] # [doc = " let a3 = *p.add(3);"] # [doc = " __m128::new(a3, a2, a1, a0)"] # [doc = " ```"] # [doc = ""] # [doc = " This corresponds to instructions `VMOVAPS` / `MOVAPS` followed by some"] # [doc = " shuffling."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loadr_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loadr_ps (p : * const f32) -> __m128 { let a = _mm_load_ps (p) ; simd_shuffle ! (a , a , [3 , 2 , 1 , 0]) }
}

macro_rules! _mm_store_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_ss_introspect!();
    # [doc = " Stores the lowest 32 bit float of `a` into memory."] # [doc = ""] # [doc = " This intrinsic corresponds to the `MOVSS` instruction."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_store_ss (p : * mut f32 , a : __m128) { * p = simd_extract ! (a , 0) ; }
}

macro_rules! _mm_store1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store1_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_store1_ps_introspect!();
    # [doc = " Stores the lowest 32 bit float of `a` repeated four times into *aligned*"] # [doc = " memory."] # [doc = ""] # [doc = " If the pointer is not aligned to a 128-bit boundary (16 bytes) a general"] # [doc = " protection fault will be triggered (fatal program crash)."] # [doc = ""] # [doc = " Functionally equivalent to the following code sequence (assuming `p`"] # [doc = " satisfies the alignment restrictions):"] # [doc = ""] # [doc = " ```text"] # [doc = " let x = a.extract(0);"] # [doc = " *p = x;"] # [doc = " *p.add(1) = x;"] # [doc = " *p.add(2) = x;"] # [doc = " *p.add(3) = x;"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store1_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_store1_ps (p : * mut f32 , a : __m128) { let b : __m128 = simd_shuffle ! (a , a , [0 , 0 , 0 , 0]) ; * (p as * mut __m128) = b ; }
}

macro_rules! _mm_store_ps1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_ps1 in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_ps1_introspect!();
    # [doc = " Alias for [`_mm_store1_ps`](fn._mm_store1_ps.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_ps1)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_store_ps1 (p : * mut f32 , a : __m128) { _mm_store1_ps (p , a) ; }
}

macro_rules! _mm_store_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_store_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_store_ps_introspect!();
    # [doc = " Stores four 32-bit floats into *aligned* memory."] # [doc = ""] # [doc = " If the pointer is not aligned to a 128-bit boundary (16 bytes) a general"] # [doc = " protection fault will be triggered (fatal program crash)."] # [doc = ""] # [doc = " Use [`_mm_storeu_ps`](fn._mm_storeu_ps.html) for potentially unaligned"] # [doc = " memory."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVAPS` / `MOVAPS`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_store_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_store_ps (p : * mut f32 , a : __m128) { * (p as * mut __m128) = a ; }
}

macro_rules! _mm_storeu_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storeu_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_storeu_ps_introspect!();
    # [doc = " Stores four 32-bit floats into memory. There are no restrictions on memory"] # [doc = " alignment. For aligned memory [`_mm_store_ps`](fn._mm_store_ps.html) may be"] # [doc = " faster."] # [doc = ""] # [doc = " This corresponds to instructions `VMOVUPS` / `MOVUPS`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storeu_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movups))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_storeu_ps (p : * mut f32 , a : __m128) { ptr :: copy_nonoverlapping (ptr :: addr_of ! (a) as * const u8 , p as * mut u8 , mem :: size_of :: < __m128 > () ,) ; }
}

macro_rules! _mm_storer_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_storer_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_storer_ps_introspect!();
    # [doc = " Stores four 32-bit floats into *aligned* memory in reverse order."] # [doc = ""] # [doc = " If the pointer is not aligned to a 128-bit boundary (16 bytes) a general"] # [doc = " protection fault will be triggered (fatal program crash)."] # [doc = ""] # [doc = " Functionally equivalent to the following code sequence (assuming `p`"] # [doc = " satisfies the alignment restrictions):"] # [doc = ""] # [doc = " ```text"] # [doc = " *p = a.extract(3);"] # [doc = " *p.add(1) = a.extract(2);"] # [doc = " *p.add(2) = a.extract(1);"] # [doc = " *p.add(3) = a.extract(0);"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_storer_ps)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (all (test , not (all (target_arch = "x86" , target_env = "msvc"))) , assert_instr (movaps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_storer_ps (p : * mut f32 , a : __m128) { let b : __m128 = simd_shuffle ! (a , a , [3 , 2 , 1 , 0]) ; * (p as * mut __m128) = b ; }
}

macro_rules! _mm_move_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_move_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_move_ss_introspect!();
    # [doc = " Returns a `__m128` with the first component from `b` and the remaining"] # [doc = " components from `a`."] # [doc = ""] # [doc = " In other words for any `a` and `b`:"] # [doc = " ```text"] # [doc = " _mm_move_ss(a, b) == a.replace(0, b.extract(0))"] # [doc = " ```"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_move_ss)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_move_ss (a : __m128 , b : __m128) -> __m128 { unsafe { simd_shuffle ! (a , b , [4 , 1 , 2 , 3]) } }
}

macro_rules! _mm_sfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sfence in module {}", module_path!());
    };
}

mkfn!{
    _mm_sfence_introspect!();
    # [doc = " Performs a serializing operation on all non-temporal (\"streaming\") store instructions that"] # [doc = " were issued by the current thread prior to this instruction."] # [doc = ""] # [doc = " Guarantees that every non-temporal store instruction that precedes this fence, in program order, is"] # [doc = " ordered before any load or store instruction which follows the fence in"] # [doc = " synchronization order."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sfence)"] # [doc = " (but note that Intel is only documenting the hardware-level concerns related to this"] # [doc = " instruction; the Intel documentation does not take into account the extra concerns that arise"] # [doc = " because the Rust memory model is different from the x86 memory model.)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using any non-temporal store intrinsic, but before any other access to the memory that the"] # [doc = " intrinsic mutates, a call to `_mm_sfence` must be performed on the thread that used the"] # [doc = " intrinsic."] # [doc = ""] # [doc = " Non-temporal stores behave very different from regular stores. For the purpose of the Rust"] # [doc = " memory model, these stores are happening asynchronously in a background thread. This means a"] # [doc = " non-temporal store can cause data races with other accesses, even other accesses on the same"] # [doc = " thread. It also means that cross-thread synchronization does not work as expected: let's say the"] # [doc = " intrinsic is called on thread T1, and T1 performs synchronization with some other thread T2. The"] # [doc = " non-temporal store acts as if it happened not in T1 but in a different thread T3, and T2 has not"] # [doc = " synchronized with T3! Calling `_mm_sfence` makes the current thread wait for and synchronize"] # [doc = " with all the non-temporal stores previously started on this thread, which means in particular"] # [doc = " that subsequent synchronization with other threads will then work as intended again."] # [doc = ""] # [doc = " The general pattern to use non-temporal stores correctly is to call `_mm_sfence` before your"] # [doc = " code jumps back to code outside your library. This ensures all stores inside your function"] # [doc = " are synchronized-before the return, and thus transitively synchronized-before everything"] # [doc = " the caller does after your function returns."] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (sfence))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_sfence () { sfence () }
}

macro_rules! _mm_getcsr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_getcsr in module {}", module_path!());
    };
}

mkfn!{
    _mm_getcsr_introspect!();
    # [doc = " Gets the unsigned 32-bit value of the MXCSR control and status register."] # [doc = ""] # [doc = " Note that Rust makes no guarantees whatsoever about the contents of this register: Rust"] # [doc = " floating-point operations may or may not result in this register getting updated with exception"] # [doc = " state, and the register can change between two invocations of this function even when no"] # [doc = " floating-point operations appear in the source code (since floating-point operations appearing"] # [doc = " earlier or later can be reordered)."] # [doc = ""] # [doc = " If you need to perform some floating-point operations and check whether they raised an"] # [doc = " exception, use an inline assembly block for the entire sequence of operations."] # [doc = ""] # [doc = " For more info see [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_getcsr)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (stmxcsr))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_getcsr` documentation - use inline assembly instead")] pub unsafe fn _mm_getcsr () -> u32 { unsafe { let mut result = 0_i32 ; stmxcsr (ptr :: addr_of_mut ! (result) as * mut i8) ; result as u32 } }
}

macro_rules! _mm_setcsr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_setcsr in module {}", module_path!());
    };
}

mkfn!{
    _mm_setcsr_introspect!();
    # [doc = " Sets the MXCSR register with the 32-bit unsigned integer value."] # [doc = ""] # [doc = " This register controls how SIMD instructions handle floating point"] # [doc = " operations. Modifying this register only affects the current thread."] # [doc = ""] # [doc = " It contains several groups of flags:"] # [doc = ""] # [doc = " * *Exception flags* report which exceptions occurred since last they were reset."] # [doc = ""] # [doc = " * *Masking flags* can be used to mask (ignore) certain exceptions. By default"] # [doc = "   these flags are all set to 1, so all exceptions are masked. When"] # [doc = "   an exception is masked, the processor simply sets the exception flag and"] # [doc = "   continues the operation. If the exception is unmasked, the flag is also set"] # [doc = "   but additionally an exception handler is invoked."] # [doc = ""] # [doc = " * *Rounding mode flags* control the rounding mode of floating point"] # [doc = "   instructions."] # [doc = ""] # [doc = " * The *denormals-are-zero mode flag* turns all numbers which would be"] # [doc = "   denormalized (exponent bits are all zeros) into zeros."] # [doc = ""] # [doc = " Note that modifying the masking flags, rounding mode, or denormals-are-zero mode flags leads to"] # [doc = " **immediate Undefined Behavior**: Rust assumes that these are always in their default state and"] # [doc = " will optimize accordingly. This even applies when the register is altered and later reset to its"] # [doc = " original value without any floating-point operations appearing in the source code between those"] # [doc = " operations (since floating-point operations appearing earlier or later can be reordered)."] # [doc = ""] # [doc = " If you need to perform some floating-point operations under a different masking flags, rounding"] # [doc = " mode, or denormals-are-zero mode, use an inline assembly block and make sure to restore the"] # [doc = " original MXCSR register state before the end of the block."] # [doc = ""] # [doc = " ## Exception Flags"] # [doc = ""] # [doc = " * `_MM_EXCEPT_INVALID`: An invalid operation was performed (e.g., dividing"] # [doc = "   Infinity by Infinity)."] # [doc = ""] # [doc = " * `_MM_EXCEPT_DENORM`: An operation attempted to operate on a denormalized"] # [doc = "   number. Mainly this can cause loss of precision."] # [doc = ""] # [doc = " * `_MM_EXCEPT_DIV_ZERO`: Division by zero occurred."] # [doc = ""] # [doc = " * `_MM_EXCEPT_OVERFLOW`: A numeric overflow exception occurred, i.e., a"] # [doc = "   result was too large to be represented (e.g., an `f32` with absolute"] # [doc = "   value greater than `2^128`)."] # [doc = ""] # [doc = " * `_MM_EXCEPT_UNDERFLOW`: A numeric underflow exception occurred, i.e., a"] # [doc = "   result was too small to be represented in a normalized way (e.g., an"] # [doc = "   `f32` with absolute value smaller than `2^-126`.)"] # [doc = ""] # [doc = " * `_MM_EXCEPT_INEXACT`: An inexact-result exception occurred (a.k.a."] # [doc = "   precision exception). This means some precision was lost due to rounding."] # [doc = "   For example, the fraction `1/3` cannot be represented accurately in a"] # [doc = "   32 or 64 bit float and computing it would cause this exception to be"] # [doc = "   raised. Precision exceptions are very common, so they are usually masked."] # [doc = ""] # [doc = " Exception flags can be read and set using the convenience functions"] # [doc = " `_MM_GET_EXCEPTION_STATE` and `_MM_SET_EXCEPTION_STATE`. For example, to"] # [doc = " check if an operation caused some overflow:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " _MM_SET_EXCEPTION_STATE(0); // clear all exception flags"] # [doc = "                             // perform calculations"] # [doc = " if _MM_GET_EXCEPTION_STATE() & _MM_EXCEPT_OVERFLOW != 0 {"] # [doc = "     // handle overflow"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ## Masking Flags"] # [doc = ""] # [doc = " There is one masking flag for each exception flag: `_MM_MASK_INVALID`,"] # [doc = " `_MM_MASK_DENORM`, `_MM_MASK_DIV_ZERO`, `_MM_MASK_OVERFLOW`,"] # [doc = " `_MM_MASK_UNDERFLOW`, `_MM_MASK_INEXACT`."] # [doc = ""] # [doc = " A single masking bit can be set via"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " _MM_SET_EXCEPTION_MASK(_MM_MASK_UNDERFLOW);"] # [doc = " ```"] # [doc = ""] # [doc = " However, since mask bits are by default all set to 1, it is more common to"] # [doc = " want to *disable* certain bits. For example, to unmask the underflow"] # [doc = " exception, use:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " _mm_setcsr(_mm_getcsr() & !_MM_MASK_UNDERFLOW); // unmask underflow"] # [doc = " exception"] # [doc = " ```"] # [doc = ""] # [doc = " Warning: an unmasked exception will cause an exception handler to be"] # [doc = " called."] # [doc = " The standard handler will simply terminate the process. So, in this case"] # [doc = " any underflow exception would terminate the current process with something"] # [doc = " like `signal: 8, SIGFPE: erroneous arithmetic operation`."] # [doc = ""] # [doc = " ## Rounding Mode"] # [doc = ""] # [doc = " The rounding mode is describe using two bits. It can be read and set using"] # [doc = " the convenience wrappers `_MM_GET_ROUNDING_MODE()` and"] # [doc = " `_MM_SET_ROUNDING_MODE(mode)`."] # [doc = ""] # [doc = " The rounding modes are:"] # [doc = ""] # [doc = " * `_MM_ROUND_NEAREST`: (default) Round to closest to the infinite precision"] # [doc = "   value. If two values are equally close, round to even (i.e., least"] # [doc = "   significant bit will be zero)."] # [doc = ""] # [doc = " * `_MM_ROUND_DOWN`: Round toward negative Infinity."] # [doc = ""] # [doc = " * `_MM_ROUND_UP`: Round toward positive Infinity."] # [doc = ""] # [doc = " * `_MM_ROUND_TOWARD_ZERO`: Round towards zero (truncate)."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " _MM_SET_ROUNDING_MODE(_MM_ROUND_DOWN)"] # [doc = " ```"] # [doc = ""] # [doc = " ## Denormals-are-zero/Flush-to-zero Mode"] # [doc = ""] # [doc = " If this bit is set, values that would be denormalized will be set to zero"] # [doc = " instead. This is turned off by default."] # [doc = ""] # [doc = " You can read and enable/disable this mode via the helper functions"] # [doc = " `_MM_GET_FLUSH_ZERO_MODE()` and `_MM_SET_FLUSH_ZERO_MODE()`:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_OFF); // turn off (default)"] # [doc = " _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_ON); // turn on"] # [doc = " ```"] # [doc = ""] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_setcsr)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (ldmxcsr))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_setcsr` documentation - use inline assembly instead")] pub unsafe fn _mm_setcsr (val : u32) { ldmxcsr (ptr :: addr_of ! (val) as * const i8) ; }
}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_INVALID : u32 = 0x0001 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_DENORM : u32 = 0x0002 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_DIV_ZERO : u32 = 0x0004 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_OVERFLOW : u32 = 0x0008 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_UNDERFLOW : u32 = 0x0010 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_INEXACT : u32 = 0x0020 ;}
mkitem!{# [doc = " See [`_MM_GET_EXCEPTION_STATE`](fn._MM_GET_EXCEPTION_STATE.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_EXCEPT_MASK : u32 = 0x003f ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_INVALID : u32 = 0x0080 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_DENORM : u32 = 0x0100 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_DIV_ZERO : u32 = 0x0200 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_OVERFLOW : u32 = 0x0400 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_UNDERFLOW : u32 = 0x0800 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_INEXACT : u32 = 0x1000 ;}
mkitem!{# [doc = " See [`_MM_GET_EXCEPTION_MASK`](fn._MM_GET_EXCEPTION_MASK.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_MASK_MASK : u32 = 0x1f80 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_ROUND_NEAREST : u32 = 0x0000 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_ROUND_DOWN : u32 = 0x2000 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_ROUND_UP : u32 = 0x4000 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_ROUND_TOWARD_ZERO : u32 = 0x6000 ;}
mkitem!{# [doc = " See [`_MM_GET_ROUNDING_MODE`](fn._MM_GET_ROUNDING_MODE.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_ROUND_MASK : u32 = 0x6000 ;}
mkitem!{# [doc = " See [`_MM_GET_FLUSH_ZERO_MODE`](fn._MM_GET_FLUSH_ZERO_MODE.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_FLUSH_ZERO_MASK : u32 = 0x8000 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_FLUSH_ZERO_ON : u32 = 0x8000 ;}
mkitem!{# [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_FLUSH_ZERO_OFF : u32 = 0x0000 ;}

macro_rules! _MM_GET_EXCEPTION_MASK_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_GET_EXCEPTION_MASK in module {}", module_path!());
    };
}

mkfn!{
    _MM_GET_EXCEPTION_MASK_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_GET_EXCEPTION_MASK)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_getcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_GET_EXCEPTION_MASK () -> u32 { _mm_getcsr () & _MM_MASK_MASK }
}

macro_rules! _MM_GET_EXCEPTION_STATE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_GET_EXCEPTION_STATE in module {}", module_path!());
    };
}

mkfn!{
    _MM_GET_EXCEPTION_STATE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_GET_EXCEPTION_STATE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_getcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_GET_EXCEPTION_STATE () -> u32 { _mm_getcsr () & _MM_EXCEPT_MASK }
}

macro_rules! _MM_GET_FLUSH_ZERO_MODE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_GET_FLUSH_ZERO_MODE in module {}", module_path!());
    };
}

mkfn!{
    _MM_GET_FLUSH_ZERO_MODE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_GET_FLUSH_ZERO_MODE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_getcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_GET_FLUSH_ZERO_MODE () -> u32 { _mm_getcsr () & _MM_FLUSH_ZERO_MASK }
}

macro_rules! _MM_GET_ROUNDING_MODE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_GET_ROUNDING_MODE in module {}", module_path!());
    };
}

mkfn!{
    _MM_GET_ROUNDING_MODE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_GET_ROUNDING_MODE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_getcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_GET_ROUNDING_MODE () -> u32 { _mm_getcsr () & _MM_ROUND_MASK }
}

macro_rules! _MM_SET_EXCEPTION_MASK_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_SET_EXCEPTION_MASK in module {}", module_path!());
    };
}

mkfn!{
    _MM_SET_EXCEPTION_MASK_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_SET_EXCEPTION_MASK)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_setcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_SET_EXCEPTION_MASK (x : u32) { _mm_setcsr ((_mm_getcsr () & ! _MM_MASK_MASK) | (x & _MM_MASK_MASK)) }
}

macro_rules! _MM_SET_EXCEPTION_STATE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_SET_EXCEPTION_STATE in module {}", module_path!());
    };
}

mkfn!{
    _MM_SET_EXCEPTION_STATE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_SET_EXCEPTION_STATE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_setcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_SET_EXCEPTION_STATE (x : u32) { _mm_setcsr ((_mm_getcsr () & ! _MM_EXCEPT_MASK) | (x & _MM_EXCEPT_MASK)) }
}

macro_rules! _MM_SET_FLUSH_ZERO_MODE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_SET_FLUSH_ZERO_MODE in module {}", module_path!());
    };
}

mkfn!{
    _MM_SET_FLUSH_ZERO_MODE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_SET_FLUSH_ZERO_MODE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_setcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_SET_FLUSH_ZERO_MODE (x : u32) { _mm_setcsr ((_mm_getcsr () & ! _MM_FLUSH_ZERO_MASK) | (x & _MM_FLUSH_ZERO_MASK)) }
}

macro_rules! _MM_SET_ROUNDING_MODE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_SET_ROUNDING_MODE in module {}", module_path!());
    };
}

mkfn!{
    _MM_SET_ROUNDING_MODE_introspect!();
    # [doc = " See [`_mm_setcsr`](fn._mm_setcsr.html)"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_SET_ROUNDING_MODE)"] # [inline] # [allow (deprecated)] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.75.0" , note = "see `_mm_setcsr` documentation - use inline assembly instead")] pub unsafe fn _MM_SET_ROUNDING_MODE (x : u32) { _mm_setcsr ((_mm_getcsr () & ! _MM_ROUND_MASK) | (x & _MM_ROUND_MASK)) }
}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_T0 : i32 = 3 ;}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_T1 : i32 = 2 ;}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_T2 : i32 = 1 ;}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_NTA : i32 = 0 ;}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_ET0 : i32 = 7 ;}
mkitem!{# [doc = " See [`_mm_prefetch`](fn._mm_prefetch.html)."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _MM_HINT_ET1 : i32 = 6 ;}

macro_rules! _mm_prefetch_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_prefetch in module {}", module_path!());
    };
}

mkfn!{
    _mm_prefetch_introspect!();
    # [doc = " Fetch the cache line that contains address `p` using the given `STRATEGY`."] # [doc = ""] # [doc = " The `STRATEGY` must be one of:"] # [doc = ""] # [doc = " * [`_MM_HINT_T0`](constant._MM_HINT_T0.html): Fetch into all levels of the"] # [doc = "   cache hierarchy."] # [doc = ""] # [doc = " * [`_MM_HINT_T1`](constant._MM_HINT_T1.html): Fetch into L2 and higher."] # [doc = ""] # [doc = " * [`_MM_HINT_T2`](constant._MM_HINT_T2.html): Fetch into L3 and higher or"] # [doc = "   an implementation-specific choice (e.g., L2 if there is no L3)."] # [doc = ""] # [doc = " * [`_MM_HINT_NTA`](constant._MM_HINT_NTA.html): Fetch data using the"] # [doc = "   non-temporal access (NTA) hint. It may be a place closer than main memory"] # [doc = "   but outside of the cache hierarchy. This is used to reduce access latency"] # [doc = "   without polluting the cache."] # [doc = ""] # [doc = " * [`_MM_HINT_ET0`](constant._MM_HINT_ET0.html) and"] # [doc = "   [`_MM_HINT_ET1`](constant._MM_HINT_ET1.html) are similar to `_MM_HINT_T0`"] # [doc = "   and `_MM_HINT_T1` but indicate an anticipation to write to the address."] # [doc = ""] # [doc = " The actual implementation depends on the particular CPU. This instruction"] # [doc = " is considered a hint, so the CPU is also free to simply ignore the request."] # [doc = ""] # [doc = " The amount of prefetched data depends on the cache line size of the"] # [doc = " specific CPU, but it will be at least 32 bytes."] # [doc = ""] # [doc = " Common caveats:"] # [doc = ""] # [doc = " * Most modern CPUs already automatically prefetch data based on predicted"] # [doc = "   access patterns."] # [doc = ""] # [doc = " * Data is usually not fetched if this would cause a TLB miss or a page"] # [doc = "   fault."] # [doc = ""] # [doc = " * Too much prefetching can cause unnecessary cache evictions."] # [doc = ""] # [doc = " * Prefetching may also fail if there are not enough memory-subsystem"] # [doc = "   resources (e.g., request buffers)."] # [doc = ""] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_prefetch)"] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (prefetcht0 , STRATEGY = _MM_HINT_T0))] # [cfg_attr (test , assert_instr (prefetcht1 , STRATEGY = _MM_HINT_T1))] # [cfg_attr (test , assert_instr (prefetcht2 , STRATEGY = _MM_HINT_T2))] # [cfg_attr (test , assert_instr (prefetchnta , STRATEGY = _MM_HINT_NTA))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_prefetch < const STRATEGY : i32 > (p : * const i8) { static_assert_uimm_bits ! (STRATEGY , 3) ; prefetch (p , (STRATEGY >> 2) & 1 , STRATEGY & 3 , 1) ; }
}

macro_rules! _mm_undefined_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_undefined_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_undefined_ps_introspect!();
    # [doc = " Returns vector of type __m128 with indeterminate elements.with indetermination elements."] # [doc = " Despite using the word \"undefined\" (following Intel's naming scheme), this non-deterministically"] # [doc = " picks some valid value and is not equivalent to [`mem::MaybeUninit`]."] # [doc = " In practice, this is typically equivalent to [`mem::zeroed`]."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_undefined_ps)"] # [inline] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_undefined_ps () -> __m128 { const { unsafe { mem :: zeroed () } } }
}

macro_rules! _MM_TRANSPOSE4_PS_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _MM_TRANSPOSE4_PS in module {}", module_path!());
    };
}

mkfn!{
    _MM_TRANSPOSE4_PS_introspect!();
    # [doc = " Transpose the 4x4 matrix formed by 4 rows of __m128 in place."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_MM_TRANSPOSE4_PS)"] # [inline] # [allow (non_snake_case)] # [target_feature (enable = "sse")] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _MM_TRANSPOSE4_PS (row0 : & mut __m128 , row1 : & mut __m128 , row2 : & mut __m128 , row3 : & mut __m128 ,) { let tmp0 = _mm_unpacklo_ps (* row0 , * row1) ; let tmp2 = _mm_unpacklo_ps (* row2 , * row3) ; let tmp1 = _mm_unpackhi_ps (* row0 , * row1) ; let tmp3 = _mm_unpackhi_ps (* row2 , * row3) ; * row0 = _mm_movelh_ps (tmp0 , tmp2) ; * row1 = _mm_movehl_ps (tmp2 , tmp0) ; * row2 = _mm_movelh_ps (tmp1 , tmp3) ; * row3 = _mm_movehl_ps (tmp3 , tmp1) ; }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse.rcp.ss"] fn rcpss (a : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.rcp.ps"] fn rcpps (a : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.rsqrt.ss"] fn rsqrtss (a : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.rsqrt.ps"] fn rsqrtps (a : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.min.ss"] fn minss (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.min.ps"] fn minps (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.max.ss"] fn maxss (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.max.ps"] fn maxps (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse.cmp.ps"] fn cmpps (a : __m128 , b : __m128 , imm8 : i8) -> __m128 ; # [link_name = "llvm.x86.sse.comieq.ss"] fn comieq_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.comilt.ss"] fn comilt_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.comile.ss"] fn comile_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.comigt.ss"] fn comigt_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.comige.ss"] fn comige_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.comineq.ss"] fn comineq_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomieq.ss"] fn ucomieq_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomilt.ss"] fn ucomilt_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomile.ss"] fn ucomile_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomigt.ss"] fn ucomigt_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomige.ss"] fn ucomige_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.ucomineq.ss"] fn ucomineq_ss (a : __m128 , b : __m128) -> i32 ; # [link_name = "llvm.x86.sse.cvtss2si"] fn cvtss2si (a : __m128) -> i32 ; # [link_name = "llvm.x86.sse.cvttss2si"] fn cvttss2si (a : __m128) -> i32 ; # [link_name = "llvm.x86.sse.cvtsi2ss"] fn cvtsi2ss (a : __m128 , b : i32) -> __m128 ; # [link_name = "llvm.x86.sse.sfence"] fn sfence () ; # [link_name = "llvm.x86.sse.stmxcsr"] fn stmxcsr (p : * mut i8) ; # [link_name = "llvm.x86.sse.ldmxcsr"] fn ldmxcsr (p : * const i8) ; # [link_name = "llvm.prefetch"] fn prefetch (p : * const i8 , rw : i32 , loc : i32 , ty : i32) ; # [link_name = "llvm.x86.sse.cmp.ss"] fn cmpss (a : __m128 , b : __m128 , imm8 : i8) -> __m128 ; }}

macro_rules! _mm_stream_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_ps_introspect!();
    # [doc = " Stores `a` into the memory at `mem_addr` using a non-temporal memory hint."] # [doc = ""] # [doc = " `mem_addr` must be aligned on a 16-byte boundary or a general-protection"] # [doc = " exception _may_ be generated."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_stream_ps)"] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse")] # [cfg_attr (test , assert_instr (movntps))] # [stable (feature = "simd_x86" , since = "1.27.0")] # [allow (clippy :: cast_ptr_alignment)] pub unsafe fn _mm_stream_ps (mem_addr : * mut f32 , a : __m128) { crate :: arch :: asm ! (vps ! ("movntps" , ",{a}") , p = in (reg) mem_addr , a = in (xmm_reg) a , options (nostack , preserves_flags) ,) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: { hint :: black_box , mem :: transmute , ptr } ;}
mkuse!{use std :: boxed ;}
mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkitem!{const NAN : f32 = f32 :: NAN ;}

macro_rules! test_mm_add_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_add_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_add_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 101.0 , 25.0 , 0.0 , - 15.0)) ; }
}

macro_rules! test_mm_add_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_add_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_add_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_add_ss () { let a = _mm_set_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_set_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_add_ss (a , b) ; assert_eq_m128 (r , _mm_set_ps (- 1.0 , 5.0 , 0.0 , - 15.0)) ; }
}

macro_rules! test_mm_sub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_sub_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_sub_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (99.0 , - 15.0 , 0.0 , - 5.0)) ; }
}

macro_rules! test_mm_sub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sub_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sub_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_sub_ss () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_sub_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (99.0 , 5.0 , 0.0 , - 10.0)) ; }
}

macro_rules! test_mm_mul_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mul_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mul_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_mul_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_mul_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (100.0 , 100.0 , 0.0 , 50.0)) ; }
}

macro_rules! test_mm_mul_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mul_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mul_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_mul_ss () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_mul_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (100.0 , 5.0 , 0.0 , - 10.0)) ; }
}

macro_rules! test_mm_div_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_div_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_div_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_div_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 2.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.2 , - 5.0) ; let r = _mm_div_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (0.01 , 0.25 , 10.0 , 2.0)) ; }
}

macro_rules! test_mm_div_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_div_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_div_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_div_ss () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_div_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (0.01 , 5.0 , 0.0 , - 10.0)) ; }
}

macro_rules! test_mm_sqrt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sqrt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sqrt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_sqrt_ss () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_sqrt_ss (a) ; let e = _mm_setr_ps (2.0 , 13.0 , 16.0 , 100.0) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_sqrt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sqrt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sqrt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_sqrt_ps () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_sqrt_ps (a) ; let e = _mm_setr_ps (2.0 , 3.6055512 , 4.0 , 10.0) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_rcp_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rcp_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rcp_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_rcp_ss () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_rcp_ss (a) ; let e = _mm_setr_ps (0.24993896 , 13.0 , 16.0 , 100.0) ; let rel_err = 0.00048828125 ; assert_approx_eq ! (get_m128 (r , 0) , get_m128 (e , 0) , 2. * rel_err) ; for i in 1 .. 4 { assert_eq ! (get_m128 (r , i) , get_m128 (e , i)) ; } }
}

macro_rules! test_mm_rcp_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rcp_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rcp_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_rcp_ps () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_rcp_ps (a) ; let e = _mm_setr_ps (0.24993896 , 0.0769043 , 0.06248474 , 0.0099983215) ; let rel_err = 0.00048828125 ; for i in 0 .. 4 { assert_approx_eq ! (get_m128 (r , i) , get_m128 (e , i) , 2. * rel_err) ; } }
}

macro_rules! test_mm_rsqrt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rsqrt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rsqrt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_rsqrt_ss () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_rsqrt_ss (a) ; let e = _mm_setr_ps (0.49987793 , 13.0 , 16.0 , 100.0) ; let rel_err = 0.00048828125 ; for i in 0 .. 4 { assert_approx_eq ! (get_m128 (r , i) , get_m128 (e , i) , 2. * rel_err) ; } }
}

macro_rules! test_mm_rsqrt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rsqrt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rsqrt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_rsqrt_ps () { let a = _mm_setr_ps (4.0 , 13.0 , 16.0 , 100.0) ; let r = _mm_rsqrt_ps (a) ; let e = _mm_setr_ps (0.49987793 , 0.2772827 , 0.24993896 , 0.099990845) ; let rel_err = 0.00048828125 ; for i in 0 .. 4 { assert_approx_eq ! (get_m128 (r , i) , get_m128 (e , i) , 2. * rel_err) ; } }
}

macro_rules! test_mm_min_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_min_ss () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_min_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 100.0 , 5.0 , 0.0 , - 10.0)) ; }
}

macro_rules! test_mm_min_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_min_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_min_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 100.0 , 5.0 , 0.0 , - 10.0)) ; let a = _mm_setr_ps (- 0.0 , 0.0 , 0.0 , 0.0) ; let b = _mm_setr_ps (0.0 , 0.0 , 0.0 , 0.0) ; let r1 : [u8 ; 16] = transmute (_mm_min_ps (a , b)) ; let r2 : [u8 ; 16] = transmute (_mm_min_ps (b , a)) ; let a : [u8 ; 16] = transmute (a) ; let b : [u8 ; 16] = transmute (b) ; assert_eq ! (r1 , b) ; assert_eq ! (r2 , a) ; assert_ne ! (a , b) ; }
}

macro_rules! test_mm_max_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_max_ss () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_max_ss (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0)) ; }
}

macro_rules! test_mm_max_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_max_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_max_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 1.0 , 20.0 , 0.0 , - 5.0)) ; let a = _mm_setr_ps (- 0.0 , 0.0 , 0.0 , 0.0) ; let b = _mm_setr_ps (0.0 , 0.0 , 0.0 , 0.0) ; let r1 : [u8 ; 16] = transmute (_mm_max_ps (a , b)) ; let r2 : [u8 ; 16] = transmute (_mm_max_ps (b , a)) ; let a : [u8 ; 16] = transmute (a) ; let b : [u8 ; 16] = transmute (b) ; assert_eq ! (r1 , b) ; assert_eq ! (r2 , a) ; assert_ne ! (a , b) ; }
}

macro_rules! test_mm_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_and_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_and_ps () { let a = transmute (u32x4 :: splat (0b0011)) ; let b = transmute (u32x4 :: splat (0b0101)) ; let r = _mm_and_ps (* black_box (& a) , * black_box (& b)) ; let e = transmute (u32x4 :: splat (0b0001)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_andnot_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_andnot_ps () { let a = transmute (u32x4 :: splat (0b0011)) ; let b = transmute (u32x4 :: splat (0b0101)) ; let r = _mm_andnot_ps (* black_box (& a) , * black_box (& b)) ; let e = transmute (u32x4 :: splat (0b0100)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_or_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_or_ps () { let a = transmute (u32x4 :: splat (0b0011)) ; let b = transmute (u32x4 :: splat (0b0101)) ; let r = _mm_or_ps (* black_box (& a) , * black_box (& b)) ; let e = transmute (u32x4 :: splat (0b0111)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_xor_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_xor_ps () { let a = transmute (u32x4 :: splat (0b0011)) ; let b = transmute (u32x4 :: splat (0b0101)) ; let r = _mm_xor_ps (* black_box (& a) , * black_box (& b)) ; let e = transmute (u32x4 :: splat (0b0110)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cmpeq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpeq_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (- 1.0 , 5.0 , 6.0 , 7.0) ; let r : u32x4 = transmute (_mm_cmpeq_ss (a , b)) ; let e : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (0) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (r , e) ; let b2 = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let r2 : u32x4 = transmute (_mm_cmpeq_ss (a , b2)) ; let e2 : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (0xffffffff) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (r2 , e2) ; }
}

macro_rules! test_mm_cmplt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmplt_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = 0u32 ; let c1 = 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmplt_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmplt_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmplt_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmple_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmple_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = 0u32 ; let c1 = ! 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmple_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmple_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmple_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpgt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpgt_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = 0u32 ; let d1 = 0u32 ; let rb : u32x4 = transmute (_mm_cmpgt_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpgt_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpgt_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpge_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpge_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = ! 0u32 ; let d1 = 0u32 ; let rb : u32x4 = transmute (_mm_cmpge_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpge_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpge_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpneq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpneq_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmpneq_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpneq_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpneq_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpnlt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnlt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnlt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnlt_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = ! 0u32 ; let d1 = 0u32 ; let rb : u32x4 = transmute (_mm_cmpnlt_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpnlt_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpnlt_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpnle_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnle_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnle_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnle_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = 0u32 ; let d1 = 0u32 ; let rb : u32x4 = transmute (_mm_cmpnle_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpnle_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpnle_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpngt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpngt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpngt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpngt_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = 0u32 ; let c1 = ! 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmpngt_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpngt_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpngt_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpnge_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnge_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnge_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnge_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (1.0 , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = 0u32 ; let c1 = 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmpnge_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpnge_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpnge_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpord_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpord_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpord_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpord_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (NAN , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = ! 0u32 ; let c1 = 0u32 ; let d1 = ! 0u32 ; let rb : u32x4 = transmute (_mm_cmpord_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpord_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpord_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpunord_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpunord_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpunord_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpunord_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (0.0 , 5.0 , 6.0 , 7.0) ; let c = _mm_setr_ps (NAN , 5.0 , 6.0 , 7.0) ; let d = _mm_setr_ps (2.0 , 5.0 , 6.0 , 7.0) ; let b1 = 0u32 ; let c1 = ! 0u32 ; let d1 = 0u32 ; let rb : u32x4 = transmute (_mm_cmpunord_ss (a , b)) ; let eb : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (b1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rb , eb) ; let rc : u32x4 = transmute (_mm_cmpunord_ss (a , c)) ; let ec : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (c1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rc , ec) ; let rd : u32x4 = transmute (_mm_cmpunord_ss (a , d)) ; let ed : u32x4 = transmute (_mm_setr_ps (f32 :: from_bits (d1) , 2.0 , 3.0 , 4.0)) ; assert_eq ! (rd , ed) ; }
}

macro_rules! test_mm_cmpeq_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpeq_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , fls , tru , fls) ; let r : u32x4 = transmute (_mm_cmpeq_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmplt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmplt_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , fls , fls , fls) ; let r : u32x4 = transmute (_mm_cmplt_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmple_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmple_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , 4.0) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , fls , tru , fls) ; let r : u32x4 = transmute (_mm_cmple_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpgt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpgt_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 42.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , tru , fls , fls) ; let r : u32x4 = transmute (_mm_cmpgt_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpge_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpge_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 42.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , tru , tru , fls) ; let r : u32x4 = transmute (_mm_cmpge_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpneq_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpneq_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , tru , fls , tru) ; let r : u32x4 = transmute (_mm_cmpneq_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpnlt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnlt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnlt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnlt_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 5.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , tru , tru , tru) ; let r : u32x4 = transmute (_mm_cmpnlt_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpnle_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnle_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnle_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnle_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 5.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , tru , fls , tru) ; let r : u32x4 = transmute (_mm_cmpnle_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpngt_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpngt_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpngt_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpngt_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 5.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , fls , tru , tru) ; let r : u32x4 = transmute (_mm_cmpngt_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpnge_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpnge_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpnge_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpnge_ps () { let a = _mm_setr_ps (10.0 , 50.0 , 1.0 , NAN) ; let b = _mm_setr_ps (15.0 , 20.0 , 1.0 , 5.0) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , fls , fls , tru) ; let r : u32x4 = transmute (_mm_cmpnge_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpord_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpord_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpord_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpord_ps () { let a = _mm_setr_ps (10.0 , 50.0 , NAN , NAN) ; let b = _mm_setr_ps (15.0 , NAN , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (tru , fls , fls , fls) ; let r : u32x4 = transmute (_mm_cmpord_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cmpunord_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpunord_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpunord_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cmpunord_ps () { let a = _mm_setr_ps (10.0 , 50.0 , NAN , NAN) ; let b = _mm_setr_ps (15.0 , NAN , 1.0 , NAN) ; let tru = ! 0u32 ; let fls = 0u32 ; let e = u32x4 :: new (fls , tru , tru , tru) ; let r : u32x4 = transmute (_mm_cmpunord_ps (a , b)) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_comieq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comieq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comieq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_comieq_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 0 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_comieq_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_comieq_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_comilt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comilt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comilt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_comilt_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [0i32 , 1 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_comilt_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_comilt_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_comile_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comile_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comile_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_comile_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 1 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_comile_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_comile_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_comigt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comigt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comigt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_comigt_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 0 , 1 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_comige_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_comige_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_comineq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_comineq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_comineq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_comineq_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [0i32 , 1 , 1 , 1] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_comineq_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_comineq_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomieq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomieq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomieq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomieq_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 0 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomieq_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomieq_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomilt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomilt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomilt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomilt_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [0i32 , 1 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomilt_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomilt_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomile_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomile_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomile_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomile_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 1 , 0 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomile_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomile_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomigt_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomigt_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomigt_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomigt_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [0i32 , 0 , 1 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomigt_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomigt_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomige_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomige_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomige_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomige_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [1i32 , 0 , 1 , 0] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomige_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomige_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_ucomineq_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ucomineq_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ucomineq_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_ucomineq_ss () { let aa = & [3.0f32 , 12.0 , 23.0 , NAN] ; let bb = & [3.0f32 , 47.5 , 1.5 , NAN] ; let ee = & [0i32 , 1 , 1 , 1] ; for i in 0 .. 4 { let a = _mm_setr_ps (aa [i] , 1.0 , 2.0 , 3.0) ; let b = _mm_setr_ps (bb [i] , 0.0 , 2.0 , 4.0) ; let r = _mm_ucomineq_ss (a , b) ; assert_eq ! (ee [i] , r , "_mm_ucomineq_ss({:?}, {:?}) = {}, expected: {} (i={})" , a , b , r , ee [i] , i) ; } }
}

macro_rules! test_mm_cvtss_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_si32_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvtss_si32 () { let inputs = & [42.0f32 , - 3.1 , 4.0e10 , 4.0e-20 , NAN , 2147483500.1] ; let result = & [42i32 , - 3 , i32 :: MIN , 0 , i32 :: MIN , 2147483520] ; for i in 0 .. inputs . len () { let x = _mm_setr_ps (inputs [i] , 1.0 , 3.0 , 4.0) ; let e = result [i] ; let r = _mm_cvtss_si32 (x) ; assert_eq ! (e , r , "TestCase #{} _mm_cvtss_si32({:?}) = {}, expected: {}" , i , x , r , e) ; } }
}

macro_rules! test_mm_cvttss_si32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttss_si32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttss_si32_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvttss_si32 () { let inputs = & [(42.0f32 , 42i32) , (- 31.4 , - 31) , (- 33.5 , - 33) , (- 34.5 , - 34) , (10.999 , 10) , (- 5.99 , - 5) , (4.0e10 , i32 :: MIN) , (4.0e-10 , 0) , (NAN , i32 :: MIN) , (2147483500.1 , 2147483520) ,] ; for (i , & (xi , e)) in inputs . iter () . enumerate () { let x = _mm_setr_ps (xi , 1.0 , 3.0 , 4.0) ; let r = _mm_cvttss_si32 (x) ; assert_eq ! (e , r , "TestCase #{} _mm_cvttss_si32({:?}) = {}, expected: {}" , i , x , r , e) ; } }
}

macro_rules! test_mm_cvtsi32_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsi32_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsi32_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvtsi32_ss () { let inputs = & [(4555i32 , 4555.0f32) , (322223333 , 322223330.0) , (- 432 , - 432.0) , (- 322223333 , - 322223330.0) ,] ; for & (x , f) in inputs . iter () { let a = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_cvtsi32_ss (a , x) ; let e = _mm_setr_ps (f , 6.0 , 7.0 , 8.0) ; assert_eq_m128 (e , r) ; } }
}

macro_rules! test_mm_cvtss_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_f32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_f32_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_cvtss_f32 () { let a = _mm_setr_ps (312.0134 , 5.0 , 6.0 , 7.0) ; assert_eq ! (_mm_cvtss_f32 (a) , 312.0134) ; }
}

macro_rules! test_mm_set_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_set_ss () { let r = _mm_set_ss (black_box (4.25)) ; assert_eq_m128 (r , _mm_setr_ps (4.25 , 0.0 , 0.0 , 0.0)) ; }
}

macro_rules! test_mm_set1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set1_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set1_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_set1_ps () { let r1 = _mm_set1_ps (black_box (4.25)) ; let r2 = _mm_set_ps1 (black_box (4.25)) ; assert_eq ! (get_m128 (r1 , 0) , 4.25) ; assert_eq ! (get_m128 (r1 , 1) , 4.25) ; assert_eq ! (get_m128 (r1 , 2) , 4.25) ; assert_eq ! (get_m128 (r1 , 3) , 4.25) ; assert_eq ! (get_m128 (r2 , 0) , 4.25) ; assert_eq ! (get_m128 (r2 , 1) , 4.25) ; assert_eq ! (get_m128 (r2 , 2) , 4.25) ; assert_eq ! (get_m128 (r2 , 3) , 4.25) ; }
}

macro_rules! test_mm_set_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_set_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_set_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_set_ps () { let r = _mm_set_ps (black_box (1.0) , black_box (2.0) , black_box (3.0) , black_box (4.0) ,) ; assert_eq ! (get_m128 (r , 0) , 4.0) ; assert_eq ! (get_m128 (r , 1) , 3.0) ; assert_eq ! (get_m128 (r , 2) , 2.0) ; assert_eq ! (get_m128 (r , 3) , 1.0) ; }
}

macro_rules! test_mm_setr_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setr_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setr_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_setr_ps () { let r = _mm_setr_ps (black_box (1.0) , black_box (2.0) , black_box (3.0) , black_box (4.0) ,) ; assert_eq_m128 (r , _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0)) ; }
}

macro_rules! test_mm_setzero_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_setzero_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_setzero_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_setzero_ps () { let r = * black_box (& _mm_setzero_ps ()) ; assert_eq_m128 (r , _mm_set1_ps (0.0)) ; }
}

macro_rules! test_MM_SHUFFLE_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_MM_SHUFFLE in module {}", module_path!());
    };
}

mkfn!{
    test_MM_SHUFFLE_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_MM_SHUFFLE () { assert_eq ! (_MM_SHUFFLE (0 , 1 , 1 , 3) , 0b00_01_01_11) ; assert_eq ! (_MM_SHUFFLE (3 , 1 , 1 , 0) , 0b11_01_01_00) ; assert_eq ! (_MM_SHUFFLE (1 , 2 , 2 , 1) , 0b01_10_10_01) ; }
}

macro_rules! test_mm_shuffle_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_shuffle_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_shuffle_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_shuffle_ps () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_shuffle_ps :: < 0b00_01_01_11 > (a , b) ; assert_eq_m128 (r , _mm_setr_ps (4.0 , 2.0 , 6.0 , 5.0)) ; }
}

macro_rules! test_mm_unpackhi_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpackhi_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpackhi_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_unpackhi_ps () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_unpackhi_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (3.0 , 7.0 , 4.0 , 8.0)) ; }
}

macro_rules! test_mm_unpacklo_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_unpacklo_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_unpacklo_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_unpacklo_ps () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_unpacklo_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (1.0 , 5.0 , 2.0 , 6.0)) ; }
}

macro_rules! test_mm_movehl_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movehl_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movehl_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_movehl_ps () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_movehl_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (7.0 , 8.0 , 3.0 , 4.0)) ; }
}

macro_rules! test_mm_movelh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movelh_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movelh_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_movelh_ps () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_movelh_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (1.0 , 2.0 , 5.0 , 6.0)) ; }
}

macro_rules! test_mm_load_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_load_ss () { let a = 42.0f32 ; let r = _mm_load_ss (ptr :: addr_of ! (a)) ; assert_eq_m128 (r , _mm_setr_ps (42.0 , 0.0 , 0.0 , 0.0)) ; }
}

macro_rules! test_mm_load1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load1_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load1_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_load1_ps () { let a = 42.0f32 ; let r = _mm_load1_ps (ptr :: addr_of ! (a)) ; assert_eq_m128 (r , _mm_setr_ps (42.0 , 42.0 , 42.0 , 42.0)) ; }
}

macro_rules! test_mm_load_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_load_ps () { let vals = & [1.0f32 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0] ; let mut p = vals . as_ptr () ; let mut fixup = 0.0f32 ; let unalignment = (p as usize) & 0xf ; if unalignment != 0 { let delta = (16 - unalignment) >> 2 ; fixup = delta as f32 ; p = p . add (delta) ; } let r = _mm_load_ps (p) ; let e = _mm_add_ps (_mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) , _mm_set1_ps (fixup)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_loadu_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_loadu_ps () { let vals = & [1.0f32 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0] ; let p = vals . as_ptr () . add (3) ; let r = _mm_loadu_ps (black_box (p)) ; assert_eq_m128 (r , _mm_setr_ps (4.0 , 5.0 , 6.0 , 7.0)) ; }
}

macro_rules! test_mm_loadr_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadr_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadr_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_loadr_ps () { let vals = & [1.0f32 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0] ; let mut p = vals . as_ptr () ; let mut fixup = 0.0f32 ; let unalignment = (p as usize) & 0xf ; if unalignment != 0 { let delta = (16 - unalignment) >> 2 ; fixup = delta as f32 ; p = p . add (delta) ; } let r = _mm_loadr_ps (p) ; let e = _mm_add_ps (_mm_setr_ps (4.0 , 3.0 , 2.0 , 1.0) , _mm_set1_ps (fixup)) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_store_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_store_ss () { let mut vals = [0.0f32 ; 8] ; let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; _mm_store_ss (vals . as_mut_ptr () . add (1) , a) ; assert_eq ! (vals [0] , 0.0) ; assert_eq ! (vals [1] , 1.0) ; assert_eq ! (vals [2] , 0.0) ; }
}

macro_rules! test_mm_store1_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store1_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store1_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_store1_ps () { let mut vals = [0.0f32 ; 8] ; let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let mut ofs = 0 ; let mut p = vals . as_mut_ptr () ; if (p as usize) & 0xf != 0 { ofs = (16 - ((p as usize) & 0xf)) >> 2 ; p = p . add (ofs) ; } _mm_store1_ps (p , * black_box (& a)) ; if ofs > 0 { assert_eq ! (vals [ofs - 1] , 0.0) ; } assert_eq ! (vals [ofs + 0] , 1.0) ; assert_eq ! (vals [ofs + 1] , 1.0) ; assert_eq ! (vals [ofs + 2] , 1.0) ; assert_eq ! (vals [ofs + 3] , 1.0) ; assert_eq ! (vals [ofs + 4] , 0.0) ; }
}

macro_rules! test_mm_store_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_store_ps () { let mut vals = [0.0f32 ; 8] ; let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let mut ofs = 0 ; let mut p = vals . as_mut_ptr () ; if (p as usize) & 0xf != 0 { ofs = (16 - ((p as usize) & 0xf)) >> 2 ; p = p . add (ofs) ; } _mm_store_ps (p , * black_box (& a)) ; if ofs > 0 { assert_eq ! (vals [ofs - 1] , 0.0) ; } assert_eq ! (vals [ofs + 0] , 1.0) ; assert_eq ! (vals [ofs + 1] , 2.0) ; assert_eq ! (vals [ofs + 2] , 3.0) ; assert_eq ! (vals [ofs + 3] , 4.0) ; assert_eq ! (vals [ofs + 4] , 0.0) ; }
}

macro_rules! test_mm_storer_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storer_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storer_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_storer_ps () { let mut vals = [0.0f32 ; 8] ; let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let mut ofs = 0 ; let mut p = vals . as_mut_ptr () ; if (p as usize) & 0xf != 0 { ofs = (16 - ((p as usize) & 0xf)) >> 2 ; p = p . add (ofs) ; } _mm_storer_ps (p , * black_box (& a)) ; if ofs > 0 { assert_eq ! (vals [ofs - 1] , 0.0) ; } assert_eq ! (vals [ofs + 0] , 4.0) ; assert_eq ! (vals [ofs + 1] , 3.0) ; assert_eq ! (vals [ofs + 2] , 2.0) ; assert_eq ! (vals [ofs + 3] , 1.0) ; assert_eq ! (vals [ofs + 4] , 0.0) ; }
}

macro_rules! test_mm_storeu_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_storeu_ps () { let mut vals = [0.0f32 ; 8] ; let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let mut ofs = 0 ; let mut p = vals . as_mut_ptr () ; if (p as usize) & 0xf == 0 { ofs = 1 ; p = p . add (1) ; } _mm_storeu_ps (p , * black_box (& a)) ; if ofs > 0 { assert_eq ! (vals [ofs - 1] , 0.0) ; } assert_eq ! (vals [ofs + 0] , 1.0) ; assert_eq ! (vals [ofs + 1] , 2.0) ; assert_eq ! (vals [ofs + 2] , 3.0) ; assert_eq ! (vals [ofs + 3] , 4.0) ; assert_eq ! (vals [ofs + 4] , 0.0) ; }
}

macro_rules! test_mm_move_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_move_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_move_ss_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_move_ss () { let a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let r = _mm_move_ss (a , b) ; let e = _mm_setr_ps (5.0 , 2.0 , 3.0 , 4.0) ; assert_eq_m128 (e , r) ; }
}

macro_rules! test_mm_movemask_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movemask_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movemask_ps_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_mm_movemask_ps () { let r = _mm_movemask_ps (_mm_setr_ps (- 1.0 , 5.0 , - 5.0 , 0.0)) ; assert_eq ! (r , 0b0101) ; let r = _mm_movemask_ps (_mm_setr_ps (- 1.0 , - 5.0 , - 5.0 , 0.0)) ; assert_eq ! (r , 0b0111) ; }
}

macro_rules! test_mm_sfence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sfence in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sfence_introspect!();
    # [simd_test (enable = "sse")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_sfence () { _mm_sfence () ; }
}

macro_rules! test_MM_TRANSPOSE4_PS_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_MM_TRANSPOSE4_PS in module {}", module_path!());
    };
}

mkfn!{
    test_MM_TRANSPOSE4_PS_introspect!();
    # [simd_test (enable = "sse")] unsafe fn test_MM_TRANSPOSE4_PS () { let mut a = _mm_setr_ps (1.0 , 2.0 , 3.0 , 4.0) ; let mut b = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; let mut c = _mm_setr_ps (9.0 , 10.0 , 11.0 , 12.0) ; let mut d = _mm_setr_ps (13.0 , 14.0 , 15.0 , 16.0) ; _MM_TRANSPOSE4_PS (& mut a , & mut b , & mut c , & mut d) ; assert_eq_m128 (a , _mm_setr_ps (1.0 , 5.0 , 9.0 , 13.0)) ; assert_eq_m128 (b , _mm_setr_ps (2.0 , 6.0 , 10.0 , 14.0)) ; assert_eq_m128 (c , _mm_setr_ps (3.0 , 7.0 , 11.0 , 15.0)) ; assert_eq_m128 (d , _mm_setr_ps (4.0 , 8.0 , 12.0 , 16.0)) ; }
}
mkitem!{mkstruct!{# [repr (align (16))] struct Memory { pub data : [f32 ; 4] , }}}

macro_rules! test_mm_stream_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_ps_introspect!();
    # [simd_test (enable = "sse")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_ps () { let a = _mm_set1_ps (7.0) ; let mut mem = Memory { data : [- 1.0 ; 4] } ; _mm_stream_ps (ptr :: addr_of_mut ! (mem . data [0]) , a) ; for i in 0 .. 4 { assert_eq ! (mem . data [i] , get_m128 (a , i)) ; } }
} 
            }}