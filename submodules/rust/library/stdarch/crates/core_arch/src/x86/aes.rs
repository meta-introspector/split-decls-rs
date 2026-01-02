mkuse!{use crate :: core_arch :: x86 :: __m128i ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.aesni.aesdec"] fn aesdec (a : __m128i , round_key : __m128i) -> __m128i ; # [link_name = "llvm.x86.aesni.aesdeclast"] fn aesdeclast (a : __m128i , round_key : __m128i) -> __m128i ; # [link_name = "llvm.x86.aesni.aesenc"] fn aesenc (a : __m128i , round_key : __m128i) -> __m128i ; # [link_name = "llvm.x86.aesni.aesenclast"] fn aesenclast (a : __m128i , round_key : __m128i) -> __m128i ; # [link_name = "llvm.x86.aesni.aesimc"] fn aesimc (a : __m128i) -> __m128i ; # [link_name = "llvm.x86.aesni.aeskeygenassist"] fn aeskeygenassist (a : __m128i , imm8 : u8) -> __m128i ; }}

macro_rules! _mm_aesdec_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aesdec_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aesdec_si128_introspect!();
    # [doc = " Performs one round of an AES decryption flow on data (state) in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aesdec_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aesdec))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aesdec_si128 (a : __m128i , round_key : __m128i) -> __m128i { unsafe { aesdec (a , round_key) } }
}

macro_rules! _mm_aesdeclast_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aesdeclast_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aesdeclast_si128_introspect!();
    # [doc = " Performs the last round of an AES decryption flow on data (state) in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aesdeclast_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aesdeclast))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aesdeclast_si128 (a : __m128i , round_key : __m128i) -> __m128i { unsafe { aesdeclast (a , round_key) } }
}

macro_rules! _mm_aesenc_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aesenc_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aesenc_si128_introspect!();
    # [doc = " Performs one round of an AES encryption flow on data (state) in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aesenc_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aesenc))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aesenc_si128 (a : __m128i , round_key : __m128i) -> __m128i { unsafe { aesenc (a , round_key) } }
}

macro_rules! _mm_aesenclast_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aesenclast_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aesenclast_si128_introspect!();
    # [doc = " Performs the last round of an AES encryption flow on data (state) in `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aesenclast_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aesenclast))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aesenclast_si128 (a : __m128i , round_key : __m128i) -> __m128i { unsafe { aesenclast (a , round_key) } }
}

macro_rules! _mm_aesimc_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aesimc_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aesimc_si128_introspect!();
    # [doc = " Performs the `InvMixColumns` transformation on `a`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aesimc_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aesimc))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aesimc_si128 (a : __m128i) -> __m128i { unsafe { aesimc (a) } }
}

macro_rules! _mm_aeskeygenassist_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_aeskeygenassist_si128 in module {}", module_path!());
    };
}

mkfn!{
    _mm_aeskeygenassist_si128_introspect!();
    # [doc = " Assist in expanding the AES cipher key."] # [doc = ""] # [doc = " Assist in expanding the AES cipher key by computing steps towards"] # [doc = " generating a round key for encryption cipher using data from `a` and an"] # [doc = " 8-bit round constant `IMM8`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_aeskeygenassist_si128)"] # [inline] # [target_feature (enable = "aes")] # [cfg_attr (test , assert_instr (aeskeygenassist , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_aeskeygenassist_si128 < const IMM8 : i32 > (a : __m128i) -> __m128i { static_assert_uimm_bits ! (IMM8 , 8) ; unsafe { aeskeygenassist (a , IMM8 as u8) } }
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

macro_rules! test_mm_aesdec_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aesdec_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aesdec_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aesdec_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let k = _mm_set_epi64x (0x1133557799bbddff , 0x0022446688aaccee) ; let e = _mm_set_epi64x (0x044e4f5176fec48f , 0xb57ecfa381da39ee) ; let r = _mm_aesdec_si128 (a , k) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_aesdeclast_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aesdeclast_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aesdeclast_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aesdeclast_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let k = _mm_set_epi64x (0x1133557799bbddff , 0x0022446688aaccee) ; let e = _mm_set_epi64x (0x36cad57d9072bf9e , 0xf210dd981fa4a493) ; let r = _mm_aesdeclast_si128 (a , k) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_aesenc_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aesenc_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aesenc_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aesenc_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let k = _mm_set_epi64x (0x1133557799bbddff , 0x0022446688aaccee) ; let e = _mm_set_epi64x (0x16ab0e57dfc442ed , 0x28e4ee1884504333) ; let r = _mm_aesenc_si128 (a , k) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_aesenclast_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aesenclast_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aesenclast_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aesenclast_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let k = _mm_set_epi64x (0x1133557799bbddff , 0x0022446688aaccee) ; let e = _mm_set_epi64x (0xb6dd7df25d7ab320 , 0x4b04f98cf4c860f8) ; let r = _mm_aesenclast_si128 (a , k) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_aesimc_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aesimc_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aesimc_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aesimc_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let e = _mm_set_epi64x (0xc66c82284ee40aa0 , 0x6633441122770055) ; let r = _mm_aesimc_si128 (a) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_aeskeygenassist_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_aeskeygenassist_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_aeskeygenassist_si128_introspect!();
    # [simd_test (enable = "aes")] unsafe fn test_mm_aeskeygenassist_si128 () { let a = _mm_set_epi64x (0x0123456789abcdef , 0x8899aabbccddeeff) ; let e = _mm_set_epi64x (0x857c266b7c266e85 , 0xeac4eea9c4eeacea) ; let r = _mm_aeskeygenassist_si128 :: < 5 > (a) ; assert_eq_m128i (r , e) ; }
} 
            }}