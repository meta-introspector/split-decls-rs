mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use crate :: intrinsics :: simd :: { simd_fma , simd_neg } ;}
mkuse!{use crate :: intrinsics :: { fmaf32 , fmaf64 } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmadd_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_fma (a , b , c) } }
}

macro_rules! _mm256_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmadd_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { simd_fma (a , b , c) } }
}

macro_rules! _mm_fmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmadd_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_fma (a , b , c) } }
}

macro_rules! _mm256_fmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmadd_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { simd_fma (a , b , c) } }
}

macro_rules! _mm_fmadd_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmadd_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmadd_sd_introspect!();
    # [doc = " Multiplies the lower double-precision (64-bit) floating-point elements in"] # [doc = " `a` and `b`, and add the intermediate result to the lower element in `c`."] # [doc = " Stores the result in the lower element of the returned value, and copy the"] # [doc = " upper element from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmadd_sd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmadd_sd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , fmaf64 (_mm_cvtsd_f64 (a) , _mm_cvtsd_f64 (b) , _mm_cvtsd_f64 (c))) } }
}

macro_rules! _mm_fmadd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmadd_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmadd_ss_introspect!();
    # [doc = " Multiplies the lower single-precision (32-bit) floating-point elements in"] # [doc = " `a` and `b`, and add the intermediate result to the lower element in `c`."] # [doc = " Stores the result in the lower element of the returned value, and copy the"] # [doc = " 3 upper elements from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmadd_ss)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmadd_ss (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , fmaf32 (_mm_cvtss_f32 (a) , _mm_cvtss_f32 (b) , _mm_cvtss_f32 (c))) } }
}

macro_rules! _mm_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmaddsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively add and subtract packed elements in `c` to/from"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmaddsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmaddsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmaddsub_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [2 , 1]) } }
}

macro_rules! _mm256_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmaddsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively add and subtract packed elements in `c` to/from"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmaddsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmaddsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmaddsub_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [4 , 1 , 6 , 3]) } }
}

macro_rules! _mm_fmaddsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmaddsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmaddsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively add and subtract packed elements in `c` to/from"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmaddsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmaddsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmaddsub_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [4 , 1 , 6 , 3]) } }
}

macro_rules! _mm256_fmaddsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmaddsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmaddsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively add and subtract packed elements in `c` to/from"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmaddsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmaddsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmaddsub_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [8 , 1 , 10 , 3 , 12 , 5 , 14 , 7]) } }
}

macro_rules! _mm_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsub_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_fma (a , b , simd_neg (c)) } }
}

macro_rules! _mm256_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmsub_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { simd_fma (a , b , simd_neg (c)) } }
}

macro_rules! _mm_fmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub213ps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsub_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_fma (a , b , simd_neg (c)) } }
}

macro_rules! _mm256_fmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub213ps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmsub_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { simd_fma (a , b , simd_neg (c)) } }
}

macro_rules! _mm_fmsub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsub_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsub_sd_introspect!();
    # [doc = " Multiplies the lower double-precision (64-bit) floating-point elements in"] # [doc = " `a` and `b`, and subtract the lower element in `c` from the intermediate"] # [doc = " result. Store the result in the lower element of the returned value, and"] # [doc = " copy the upper element from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsub_sd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsub_sd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , fmaf64 (_mm_cvtsd_f64 (a) , _mm_cvtsd_f64 (b) , - _mm_cvtsd_f64 (c))) } }
}

macro_rules! _mm_fmsub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsub_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsub_ss_introspect!();
    # [doc = " Multiplies the lower single-precision (32-bit) floating-point elements in"] # [doc = " `a` and `b`,  and subtract the lower element in `c` from the intermediate"] # [doc = " result. Store the result in the lower element of the returned value, and"] # [doc = " copy the 3 upper elements from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsub_ss)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsub_ss (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , fmaf32 (_mm_cvtss_f32 (a) , _mm_cvtss_f32 (b) , - _mm_cvtss_f32 (c))) } }
}

macro_rules! _mm_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsubadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively subtract and add packed elements in `c` from/to"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsubadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsubadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsubadd_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [0 , 3]) } }
}

macro_rules! _mm256_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmsubadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively subtract and add packed elements in `c` from/to"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmsubadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsubadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmsubadd_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [0 , 5 , 2 , 7]) } }
}

macro_rules! _mm_fmsubadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fmsubadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fmsubadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively subtract and add packed elements in `c` from/to"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fmsubadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsubadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fmsubadd_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [0 , 5 , 2 , 7]) } }
}

macro_rules! _mm256_fmsubadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fmsubadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fmsubadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and alternatively subtract and add packed elements in `c` from/to"] # [doc = " the intermediate result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fmsubadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfmsubadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fmsubadd_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { let add = simd_fma (a , b , c) ; let sub = simd_fma (a , b , simd_neg (c)) ; simd_shuffle ! (add , sub , [0 , 9 , 2 , 11 , 4 , 13 , 6 , 15]) } }
}

macro_rules! _mm_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the negated intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmadd_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_fma (simd_neg (a) , b , c) } }
}

macro_rules! _mm256_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fnmadd_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the negated intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fnmadd_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fnmadd_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { simd_fma (simd_neg (a) , b , c) } }
}

macro_rules! _mm_fnmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the negated intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmadd_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_fma (simd_neg (a) , b , c) } }
}

macro_rules! _mm256_fnmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fnmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fnmadd_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and add the negated intermediate result to packed elements in `c`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fnmadd_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fnmadd_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { simd_fma (simd_neg (a) , b , c) } }
}

macro_rules! _mm_fnmadd_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmadd_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmadd_sd_introspect!();
    # [doc = " Multiplies the lower double-precision (64-bit) floating-point elements in"] # [doc = " `a` and `b`, and add the negated intermediate result to the lower element"] # [doc = " in `c`. Store the result in the lower element of the returned value, and"] # [doc = " copy the upper element from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmadd_sd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmadd_sd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , fmaf64 (_mm_cvtsd_f64 (a) , - _mm_cvtsd_f64 (b) , _mm_cvtsd_f64 (c))) } }
}

macro_rules! _mm_fnmadd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmadd_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmadd_ss_introspect!();
    # [doc = " Multiplies the lower single-precision (32-bit) floating-point elements in"] # [doc = " `a` and `b`, and add the negated intermediate result to the lower element"] # [doc = " in `c`. Store the result in the lower element of the returned value, and"] # [doc = " copy the 3 upper elements from `a` to the upper elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmadd_ss)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmadd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmadd_ss (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , fmaf32 (_mm_cvtss_f32 (a) , - _mm_cvtss_f32 (b) , _mm_cvtss_f32 (c))) } }
}

macro_rules! _mm_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the negated intermediate"] # [doc = " result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmsub_pd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_fma (simd_neg (a) , b , simd_neg (c)) } }
}

macro_rules! _mm256_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fnmsub_pd_introspect!();
    # [doc = " Multiplies packed double-precision (64-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the negated intermediate"] # [doc = " result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fnmsub_pd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fnmsub_pd (a : __m256d , b : __m256d , c : __m256d) -> __m256d { unsafe { simd_fma (simd_neg (a) , b , simd_neg (c)) } }
}

macro_rules! _mm_fnmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the negated intermediate"] # [doc = " result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmsub_ps (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_fma (simd_neg (a) , b , simd_neg (c)) } }
}

macro_rules! _mm256_fnmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fnmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fnmsub_ps_introspect!();
    # [doc = " Multiplies packed single-precision (32-bit) floating-point elements in `a`"] # [doc = " and `b`, and subtract packed elements in `c` from the negated intermediate"] # [doc = " result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fnmsub_ps)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm256_fnmsub_ps (a : __m256 , b : __m256 , c : __m256) -> __m256 { unsafe { simd_fma (simd_neg (a) , b , simd_neg (c)) } }
}

macro_rules! _mm_fnmsub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmsub_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmsub_sd_introspect!();
    # [doc = " Multiplies the lower double-precision (64-bit) floating-point elements in"] # [doc = " `a` and `b`, and subtract packed elements in `c` from the negated"] # [doc = " intermediate result. Store the result in the lower element of the returned"] # [doc = " value, and copy the upper element from `a` to the upper elements of the"] # [doc = " result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmsub_sd)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmsub_sd (a : __m128d , b : __m128d , c : __m128d) -> __m128d { unsafe { simd_insert ! (a , 0 , fmaf64 (_mm_cvtsd_f64 (a) , - _mm_cvtsd_f64 (b) , - _mm_cvtsd_f64 (c))) } }
}

macro_rules! _mm_fnmsub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fnmsub_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_fnmsub_ss_introspect!();
    # [doc = " Multiplies the lower single-precision (32-bit) floating-point elements in"] # [doc = " `a` and `b`, and subtract packed elements in `c` from the negated"] # [doc = " intermediate result. Store the result in the lower element of the"] # [doc = " returned value, and copy the 3 upper elements from `a` to the upper"] # [doc = " elements of the result."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fnmsub_ss)"] # [inline] # [target_feature (enable = "fma")] # [cfg_attr (test , assert_instr (vfnmsub))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_fnmsub_ss (a : __m128 , b : __m128 , c : __m128) -> __m128 { unsafe { simd_insert ! (a , 0 , fmaf32 (_mm_cvtss_f32 (a) , - _mm_cvtss_f32 (b) , - _mm_cvtss_f32 (c))) } }
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

macro_rules! test_mm_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmadd_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (9. , 15.) ; assert_eq_m128d (_mm_fmadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmadd_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (9. , 15. , 22. , 15.) ; assert_eq_m256d (_mm256_fmadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmadd_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (9. , 15. , 22. , 15.) ; assert_eq_m128 (_mm_fmadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmadd_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (9. , 15. , 22. , 15. , - 5. , - 49. , - 2. , - 31.) ; assert_eq_m256 (_mm256_fmadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fmadd_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmadd_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmadd_sd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmadd_sd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (9. , 2.) ; assert_eq_m128d (_mm_fmadd_sd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmadd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmadd_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmadd_ss_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmadd_ss () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (9. , 2. , 3. , 4.) ; assert_eq_m128 (_mm_fmadd_ss (a , b , c) , r) ; }
}

macro_rules! test_mm_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmaddsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmaddsub_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (1. , 15.) ; assert_eq_m128d (_mm_fmaddsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmaddsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmaddsub_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (1. , 15. , 20. , 15.) ; assert_eq_m256d (_mm256_fmaddsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmaddsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmaddsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmaddsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmaddsub_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (1. , 15. , 20. , 15.) ; assert_eq_m128 (_mm_fmaddsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmaddsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmaddsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmaddsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmaddsub_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (1. , 15. , 20. , 15. , 5. , - 49. , 2. , - 31.) ; assert_eq_m256 (_mm256_fmaddsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsub_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (1. , - 3.) ; assert_eq_m128d (_mm_fmsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmsub_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (1. , - 3. , 20. , 1.) ; assert_eq_m256d (_mm256_fmsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsub_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (1. , - 3. , 20. , 1.) ; assert_eq_m128 (_mm_fmsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmsub_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (1. , - 3. , 20. , 1. , 5. , - 71. , 2. , - 25.) ; assert_eq_m256 (_mm256_fmsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsub_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsub_sd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsub_sd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (1. , 2.) ; assert_eq_m128d (_mm_fmsub_sd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsub_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsub_ss_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsub_ss () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (1. , 2. , 3. , 4.) ; assert_eq_m128 (_mm_fmsub_ss (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsubadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsubadd_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (9. , - 3.) ; assert_eq_m128d (_mm_fmsubadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmsubadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmsubadd_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (9. , - 3. , 22. , 1.) ; assert_eq_m256d (_mm256_fmsubadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fmsubadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fmsubadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fmsubadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fmsubadd_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (9. , - 3. , 22. , 1.) ; assert_eq_m128 (_mm_fmsubadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fmsubadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fmsubadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fmsubadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fmsubadd_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (9. , - 3. , 22. , 1. , - 5. , - 71. , - 2. , - 25.) ; assert_eq_m256 (_mm256_fmsubadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmadd_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (- 1. , 3.) ; assert_eq_m128d (_mm_fnmadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fnmadd_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fnmadd_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (- 1. , 3. , - 20. , - 1.) ; assert_eq_m256d (_mm256_fnmadd_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmadd_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (- 1. , 3. , - 20. , - 1.) ; assert_eq_m128 (_mm_fnmadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fnmadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fnmadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fnmadd_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fnmadd_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (- 1. , 3. , - 20. , - 1. , - 5. , 71. , - 2. , 25.) ; assert_eq_m256 (_mm256_fnmadd_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmadd_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmadd_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmadd_sd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmadd_sd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (- 1. , 2.) ; assert_eq_m128d (_mm_fnmadd_sd (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmadd_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmadd_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmadd_ss_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmadd_ss () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (- 1. , 2. , 3. , 4.) ; assert_eq_m128 (_mm_fnmadd_ss (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmsub_pd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (- 9. , - 15.) ; assert_eq_m128d (_mm_fnmsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm256_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fnmsub_pd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fnmsub_pd () { let a = _mm256_setr_pd (1. , 2. , 3. , 4.) ; let b = _mm256_setr_pd (5. , 3. , 7. , 2.) ; let c = _mm256_setr_pd (4. , 9. , 1. , 7.) ; let r = _mm256_setr_pd (- 9. , - 15. , - 22. , - 15.) ; assert_eq_m256d (_mm256_fnmsub_pd (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmsub_ps () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (- 9. , - 15. , - 22. , - 15.) ; assert_eq_m128 (_mm_fnmsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm256_fnmsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fnmsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fnmsub_ps_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm256_fnmsub_ps () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 0. , 10. , - 1. , - 2.) ; let b = _mm256_setr_ps (5. , 3. , 7. , 2. , 4. , - 6. , 0. , 14.) ; let c = _mm256_setr_ps (4. , 9. , 1. , 7. , - 5. , 11. , - 2. , - 3.) ; let r = _mm256_setr_ps (- 9. , - 15. , - 22. , - 15. , 5. , 49. , 2. , 31.) ; assert_eq_m256 (_mm256_fnmsub_ps (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmsub_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmsub_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmsub_sd_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmsub_sd () { let a = _mm_setr_pd (1. , 2.) ; let b = _mm_setr_pd (5. , 3.) ; let c = _mm_setr_pd (4. , 9.) ; let r = _mm_setr_pd (- 9. , 2.) ; assert_eq_m128d (_mm_fnmsub_sd (a , b , c) , r) ; }
}

macro_rules! test_mm_fnmsub_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fnmsub_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fnmsub_ss_introspect!();
    # [simd_test (enable = "fma")] unsafe fn test_mm_fnmsub_ss () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let b = _mm_setr_ps (5. , 3. , 7. , 2.) ; let c = _mm_setr_ps (4. , 9. , 1. , 7.) ; let r = _mm_setr_ps (- 9. , 2. , 3. , 4.) ; assert_eq_m128 (_mm_fnmsub_ss (a , b , c) , r) ; }
} 
            }}