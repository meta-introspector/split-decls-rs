mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkuse!{use crate :: intrinsics :: simd :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_addsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_addsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_addsub_ps_introspect!();
    # [doc = " Alternatively add and subtract packed single-precision (32-bit)"] # [doc = " floating-point elements in `a` to/from packed elements in `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_addsub_ps)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (addsubps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_addsub_ps (a : __m128 , b : __m128) -> __m128 { unsafe { let a = a . as_f32x4 () ; let b = b . as_f32x4 () ; let add = simd_add (a , b) ; let sub = simd_sub (a , b) ; simd_shuffle ! (add , sub , [4 , 1 , 6 , 3]) } }
}

macro_rules! _mm_addsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_addsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_addsub_pd_introspect!();
    # [doc = " Alternatively add and subtract packed double-precision (64-bit)"] # [doc = " floating-point elements in `a` to/from packed elements in `b`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_addsub_pd)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (addsubpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_addsub_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { let a = a . as_f64x2 () ; let b = b . as_f64x2 () ; let add = simd_add (a , b) ; let sub = simd_sub (a , b) ; simd_shuffle ! (add , sub , [2 , 1]) } }
}

macro_rules! _mm_hadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hadd_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_hadd_pd_introspect!();
    # [doc = " Horizontally adds adjacent pairs of double-precision (64-bit)"] # [doc = " floating-point elements in `a` and `b`, and pack the results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hadd_pd)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (haddpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hadd_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { haddpd (a , b) } }
}

macro_rules! _mm_hadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hadd_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_hadd_ps_introspect!();
    # [doc = " Horizontally adds adjacent pairs of single-precision (32-bit)"] # [doc = " floating-point elements in `a` and `b`, and pack the results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hadd_ps)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (haddps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hadd_ps (a : __m128 , b : __m128) -> __m128 { unsafe { haddps (a , b) } }
}

macro_rules! _mm_hsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hsub_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_hsub_pd_introspect!();
    # [doc = " Horizontally subtract adjacent pairs of double-precision (64-bit)"] # [doc = " floating-point elements in `a` and `b`, and pack the results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hsub_pd)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (hsubpd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hsub_pd (a : __m128d , b : __m128d) -> __m128d { unsafe { hsubpd (a , b) } }
}

macro_rules! _mm_hsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_hsub_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_hsub_ps_introspect!();
    # [doc = " Horizontally adds adjacent pairs of single-precision (32-bit)"] # [doc = " floating-point elements in `a` and `b`, and pack the results."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_hsub_ps)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (hsubps))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_hsub_ps (a : __m128 , b : __m128) -> __m128 { unsafe { hsubps (a , b) } }
}

macro_rules! _mm_lddqu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_lddqu_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_lddqu_si128_introspect!();
    # [doc = " Loads 128-bits of integer data from unaligned memory."] # [doc = " This intrinsic may perform better than `_mm_loadu_si128`"] # [doc = " when the data crosses a cache line boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_lddqu_si128)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (lddqu))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_lddqu_si128 (mem_addr : * const __m128i) -> __m128i { transmute (lddqu (mem_addr as * const _)) }
}

macro_rules! _mm_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_movedup_pd_introspect!();
    # [doc = " Duplicate the low double-precision (64-bit) floating-point element"] # [doc = " from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movedup_pd)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (movddup))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movedup_pd (a : __m128d) -> __m128d { unsafe { simd_shuffle ! (a , a , [0 , 0]) } }
}

macro_rules! _mm_loaddup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_loaddup_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_loaddup_pd_introspect!();
    # [doc = " Loads a double-precision (64-bit) floating-point element from memory"] # [doc = " into both elements of return vector."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_loaddup_pd)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (movddup))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_loaddup_pd (mem_addr : * const f64) -> __m128d { _mm_load1_pd (mem_addr) }
}

macro_rules! _mm_movehdup_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movehdup_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_movehdup_ps_introspect!();
    # [doc = " Duplicate odd-indexed single-precision (32-bit) floating-point elements"] # [doc = " from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movehdup_ps)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (movshdup))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_movehdup_ps (a : __m128) -> __m128 { unsafe { simd_shuffle ! (a , a , [1 , 1 , 3 , 3]) } }
}

macro_rules! _mm_moveldup_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_moveldup_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_moveldup_ps_introspect!();
    # [doc = " Duplicate even-indexed single-precision (32-bit) floating-point elements"] # [doc = " from `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_moveldup_ps)"] # [inline] # [target_feature (enable = "sse3")] # [cfg_attr (test , assert_instr (movsldup))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_moveldup_ps (a : __m128) -> __m128 { unsafe { simd_shuffle ! (a , a , [0 , 0 , 2 , 2]) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse3.hadd.pd"] fn haddpd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse3.hadd.ps"] fn haddps (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse3.hsub.pd"] fn hsubpd (a : __m128d , b : __m128d) -> __m128d ; # [link_name = "llvm.x86.sse3.hsub.ps"] fn hsubps (a : __m128 , b : __m128) -> __m128 ; # [link_name = "llvm.x86.sse3.ldu.dq"] fn lddqu (mem_addr : * const i8) -> i8x16 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_mm_addsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_addsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_addsub_ps_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_addsub_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_addsub_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (99.0 , 25.0 , 0.0 , - 15.0)) ; }
}

macro_rules! test_mm_addsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_addsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_addsub_pd_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_addsub_pd () { let a = _mm_setr_pd (- 1.0 , 5.0) ; let b = _mm_setr_pd (- 100.0 , 20.0) ; let r = _mm_addsub_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (99.0 , 25.0)) ; }
}

macro_rules! test_mm_hadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hadd_pd_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_hadd_pd () { let a = _mm_setr_pd (- 1.0 , 5.0) ; let b = _mm_setr_pd (- 100.0 , 20.0) ; let r = _mm_hadd_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (4.0 , - 80.0)) ; }
}

macro_rules! test_mm_hadd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hadd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hadd_ps_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_hadd_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_hadd_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (4.0 , - 10.0 , - 80.0 , - 5.0)) ; }
}

macro_rules! test_mm_hsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hsub_pd_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_hsub_pd () { let a = _mm_setr_pd (- 1.0 , 5.0) ; let b = _mm_setr_pd (- 100.0 , 20.0) ; let r = _mm_hsub_pd (a , b) ; assert_eq_m128d (r , _mm_setr_pd (- 6.0 , - 120.0)) ; }
}

macro_rules! test_mm_hsub_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_hsub_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_hsub_ps_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_hsub_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let b = _mm_setr_ps (- 100.0 , 20.0 , 0.0 , - 5.0) ; let r = _mm_hsub_ps (a , b) ; assert_eq_m128 (r , _mm_setr_ps (- 6.0 , 10.0 , - 120.0 , 5.0)) ; }
}

macro_rules! test_mm_lddqu_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_lddqu_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_lddqu_si128_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_lddqu_si128 () { # [rustfmt :: skip] let a = _mm_setr_epi8 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; let r = _mm_lddqu_si128 (& a) ; assert_eq_m128i (a , r) ; }
}

macro_rules! test_mm_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movedup_pd_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_movedup_pd () { let a = _mm_setr_pd (- 1.0 , 5.0) ; let r = _mm_movedup_pd (a) ; assert_eq_m128d (r , _mm_setr_pd (- 1.0 , - 1.0)) ; }
}

macro_rules! test_mm_movehdup_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movehdup_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movehdup_ps_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_movehdup_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let r = _mm_movehdup_ps (a) ; assert_eq_m128 (r , _mm_setr_ps (5.0 , 5.0 , - 10.0 , - 10.0)) ; }
}

macro_rules! test_mm_moveldup_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_moveldup_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_moveldup_ps_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_moveldup_ps () { let a = _mm_setr_ps (- 1.0 , 5.0 , 0.0 , - 10.0) ; let r = _mm_moveldup_ps (a) ; assert_eq_m128 (r , _mm_setr_ps (- 1.0 , - 1.0 , 0.0 , 0.0)) ; }
}

macro_rules! test_mm_loaddup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loaddup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loaddup_pd_introspect!();
    # [simd_test (enable = "sse3")] unsafe fn test_mm_loaddup_pd () { let d = - 5.0 ; let r = _mm_loaddup_pd (& d) ; assert_eq_m128d (r , _mm_setr_pd (d , d)) ; }
} 
            }}