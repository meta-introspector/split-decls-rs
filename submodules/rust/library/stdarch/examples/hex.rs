mkuse!{use std :: { io :: { self , Read } , str , } ;}
mkuse!{# [cfg (target_arch = "x86")] use core_arch :: arch :: x86 :: * ;}
mkuse!{# [cfg (target_arch = "x86_64")] use core_arch :: arch :: x86_64 :: * ;}
mkuse!{# [cfg (target_arch = "x86")] use std :: is_x86_feature_detected ;}
mkuse!{# [cfg (target_arch = "x86_64")] use std :: is_x86_feature_detected ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let mut input = Vec :: new () ; io :: stdin () . read_to_end (& mut input) . unwrap () ; let mut dst = vec ! [0 ; 2 * input . len ()] ; let s = hex_encode (& input , & mut dst) . unwrap () ; println ! ("{s}") ; }
}

macro_rules! hex_encode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_introspect!();
    fn hex_encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , usize > { let len = src . len () . checked_mul (2) . unwrap () ; if dst . len () < len { return Err (len) ; } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if is_x86_feature_detected ! ("avx2") { return unsafe { hex_encode_avx2 (src , dst) } ; } if is_x86_feature_detected ! ("sse4.1") { return unsafe { hex_encode_sse41 (src , dst) } ; } } # [cfg (target_arch = "wasm32")] { if true { return hex_encode_simd128 (src , dst) ; } } hex_encode_fallback (src , dst) }
}

macro_rules! hex_encode_avx2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode_avx2 in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_avx2_introspect!();
    # [target_feature (enable = "avx2")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn hex_encode_avx2 < 'a > (mut src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , usize > { assert ! (dst . len () >= src . len () . checked_mul (2) . unwrap ()) ; let ascii_zero = _mm256_set1_epi8 (b'0' as i8) ; let nines = _mm256_set1_epi8 (9) ; let ascii_a = _mm256_set1_epi8 ((b'a' - 9 - 1) as i8) ; let and4bits = _mm256_set1_epi8 (0xf) ; let mut i = 0_usize ; while src . len () >= 32 { let invec = unsafe { _mm256_loadu_si256 (src . as_ptr () as * const _) } ; let masked1 = _mm256_and_si256 (invec , and4bits) ; let masked2 = _mm256_and_si256 (_mm256_srli_epi64 (invec , 4) , and4bits) ; let cmpmask1 = _mm256_cmpgt_epi8 (masked1 , nines) ; let cmpmask2 = _mm256_cmpgt_epi8 (masked2 , nines) ; let masked1 = _mm256_add_epi8 (masked1 , _mm256_blendv_epi8 (ascii_zero , ascii_a , cmpmask1)) ; let masked2 = _mm256_add_epi8 (masked2 , _mm256_blendv_epi8 (ascii_zero , ascii_a , cmpmask2)) ; let res1 = _mm256_unpacklo_epi8 (masked2 , masked1) ; let res2 = _mm256_unpackhi_epi8 (masked2 , masked1) ; unsafe { let base = dst . as_mut_ptr () . add (i * 2) ; let base1 = base . add (0) as * mut _ ; let base2 = base . add (16) as * mut _ ; let base3 = base . add (32) as * mut _ ; let base4 = base . add (48) as * mut _ ; _mm256_storeu2_m128i (base3 , base1 , res1) ; _mm256_storeu2_m128i (base4 , base2 , res2) ; } src = & src [32 ..] ; i += 32 ; } let _ = hex_encode_sse41 (src , & mut dst [i * 2 ..]) ; unsafe { Ok (str :: from_utf8_unchecked (& dst [.. src . len () * 2 + i * 2])) } }
}

macro_rules! hex_encode_sse41_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode_sse41 in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_sse41_introspect!();
    # [target_feature (enable = "sse4.1")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn hex_encode_sse41 < 'a > (mut src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , usize > { assert ! (dst . len () >= src . len () . checked_mul (2) . unwrap ()) ; let ascii_zero = _mm_set1_epi8 (b'0' as i8) ; let nines = _mm_set1_epi8 (9) ; let ascii_a = _mm_set1_epi8 ((b'a' - 9 - 1) as i8) ; let and4bits = _mm_set1_epi8 (0xf) ; let mut i = 0_usize ; while src . len () >= 16 { let invec = unsafe { _mm_loadu_si128 (src . as_ptr () as * const _) } ; let masked1 = _mm_and_si128 (invec , and4bits) ; let masked2 = _mm_and_si128 (_mm_srli_epi64 (invec , 4) , and4bits) ; let cmpmask1 = _mm_cmpgt_epi8 (masked1 , nines) ; let cmpmask2 = _mm_cmpgt_epi8 (masked2 , nines) ; let masked1 = _mm_add_epi8 (masked1 , _mm_blendv_epi8 (ascii_zero , ascii_a , cmpmask1)) ; let masked2 = _mm_add_epi8 (masked2 , _mm_blendv_epi8 (ascii_zero , ascii_a , cmpmask2)) ; let res1 = _mm_unpacklo_epi8 (masked2 , masked1) ; let res2 = _mm_unpackhi_epi8 (masked2 , masked1) ; unsafe { _mm_storeu_si128 (dst . as_mut_ptr () . add (i * 2) as * mut _ , res1) ; _mm_storeu_si128 (dst . as_mut_ptr () . add (i * 2 + 16) as * mut _ , res2) ; } src = & src [16 ..] ; i += 16 ; } let _ = hex_encode_fallback (src , & mut dst [i * 2 ..]) ; unsafe { Ok (str :: from_utf8_unchecked (& dst [.. src . len () * 2 + i * 2])) } }
}

macro_rules! hex_encode_simd128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode_simd128 in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_simd128_introspect!();
    # [cfg (target_arch = "wasm32")] # [target_feature (enable = "simd128")] fn hex_encode_simd128 < 'a > (mut src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , usize > { assert ! (dst . len () >= src . len () . checked_mul (2) . unwrap ()) ; use core_arch :: arch :: wasm32 :: * ; let ascii_zero = u8x16_splat (b'0') ; let nines = u8x16_splat (9) ; let ascii_a = u8x16_splat (b'a' - 9 - 1) ; let and4bits = u8x16_splat (0xf) ; let mut i = 0_usize ; while src . len () >= 16 { let invec = unsafe { v128_load (src . as_ptr () as * const _) } ; let masked1 = v128_and (invec , and4bits) ; let masked2 = v128_and (u8x16_shr (invec , 4) , and4bits) ; let cmpmask1 = u8x16_gt (masked1 , nines) ; let cmpmask2 = u8x16_gt (masked2 , nines) ; let masked1 = u8x16_add (masked1 , v128_bitselect (ascii_a , ascii_zero , cmpmask1)) ; let masked2 = u8x16_add (masked2 , v128_bitselect (ascii_a , ascii_zero , cmpmask2)) ; let res1 = u8x16_shuffle :: < 0 , 16 , 1 , 17 , 2 , 18 , 3 , 19 , 4 , 20 , 5 , 21 , 6 , 22 , 7 , 23 > (masked2 , masked1 ,) ; let res2 = u8x16_shuffle :: < 8 , 24 , 9 , 25 , 10 , 26 , 11 , 27 , 12 , 28 , 13 , 29 , 14 , 30 , 15 , 31 > (masked2 , masked1 ,) ; unsafe { v128_store (dst . as_mut_ptr () . add (i * 2) as * mut _ , res1) ; v128_store (dst . as_mut_ptr () . add (i * 2 + 16) as * mut _ , res2) ; } src = & src [16 ..] ; i += 16 ; } let _ = hex_encode_fallback (src , & mut dst [i * 2 ..]) ; unsafe { Ok (str :: from_utf8_unchecked (& dst [.. src . len () * 2 + i * 2])) } }
}

macro_rules! hex_encode_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hex_encode_fallback in module {}", module_path!());
    };
}

mkfn!{
    hex_encode_fallback_introspect!();
    fn hex_encode_fallback < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , usize > { fn hex (byte : u8) -> u8 { static TABLE : & [u8] = b"0123456789abcdef" ; TABLE [byte as usize] } for (byte , slots) in src . iter () . zip (dst . chunks_mut (2)) { slots [0] = hex ((* byte >> 4) & 0xf) ; slots [1] = hex (* byte & 0xf) ; } unsafe { Ok (str :: from_utf8_unchecked (& dst [.. src . len () * 2])) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    fn test (input : & [u8] , output : & str) { let tmp = | | vec ! [0 ; input . len () * 2] ; assert_eq ! (hex_encode_fallback (input , & mut tmp ()) . unwrap () , output) ; assert_eq ! (hex_encode (input , & mut tmp ()) . unwrap () , output) ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] unsafe { if self :: is_x86_feature_detected ! ("avx2") { assert_eq ! (hex_encode_avx2 (input , & mut tmp ()) . unwrap () , output) ; } if self :: is_x86_feature_detected ! ("sse4.1") { assert_eq ! (hex_encode_sse41 (input , & mut tmp ()) . unwrap () , output) ; } } }
}

macro_rules! empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function empty in module {}", module_path!());
    };
}

mkfn!{
    empty_introspect!();
    # [test] fn empty () { test (b"" , "") ; }
}

macro_rules! big_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function big in module {}", module_path!());
    };
}

mkfn!{
    big_introspect!();
    # [test] fn big () { test (& [0 ; 1024] , & "0" . repeat (2048)) ; }
}

macro_rules! odd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function odd in module {}", module_path!());
    };
}

mkfn!{
    odd_introspect!();
    # [test] fn odd () { test (& [0 ; 313] , & "0" . repeat (313 * 2)) ; }
}

macro_rules! avx_works_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function avx_works in module {}", module_path!());
    };
}

mkfn!{
    avx_works_introspect!();
    # [test] fn avx_works () { let mut input = [0 ; 33] ; input [4] = 3 ; input [16] = 3 ; input [17] = 0x30 ; input [21] = 1 ; input [31] = 0x24 ; test (& input , "\
             0000000003000000\
             0000000000000000\
             0330000000010000\
             0000000000000024\
             00\
             " ,) ; }
}
mkitem!{quickcheck :: quickcheck ! { fn encode_equals_fallback (input : Vec < u8 >) -> bool { let mut space1 = vec ! [0 ; input . len () * 2] ; let mut space2 = vec ! [0 ; input . len () * 2] ; let a = hex_encode (& input , & mut space1) . unwrap () ; let b = hex_encode_fallback (& input , & mut space2) . unwrap () ; a == b } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn avx_equals_fallback (input : Vec < u8 >) -> bool { if ! self :: is_x86_feature_detected ! ("avx2") { return true } let mut space1 = vec ! [0 ; input . len () * 2] ; let mut space2 = vec ! [0 ; input . len () * 2] ; let a = unsafe { hex_encode_avx2 (& input , & mut space1) . unwrap () } ; let b = hex_encode_fallback (& input , & mut space2) . unwrap () ; a == b } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn sse41_equals_fallback (input : Vec < u8 >) -> bool { if ! self :: is_x86_feature_detected ! ("avx2") { return true } let mut space1 = vec ! [0 ; input . len () * 2] ; let mut space2 = vec ! [0 ; input . len () * 2] ; let a = unsafe { hex_encode_sse41 (& input , & mut space1) . unwrap () } ; let b = hex_encode_fallback (& input , & mut space2) . unwrap () ; a == b } }} 
            }}
mkmod!{benches, { 
                getname!(benches);
                getsrc!(benches);
                getpath!(benches);
                get_deps!(benches);
                get_crates!(benches);
                mkinclude!(benches);
                mkitem!{extern crate rand ;}
mkitem!{extern crate test ;}
mkuse!{use self :: rand :: Rng ;}
mkuse!{use super :: * ;}
mkitem!{const SMALL_LEN : usize = 117 ;}
mkitem!{const LARGE_LEN : usize = 1 * 1024 * 1024 ;}

macro_rules! doit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function doit in module {}", module_path!());
    };
}

mkfn!{
    doit_introspect!();
    fn doit (b : & mut test :: Bencher , len : usize , f : for < 'a > unsafe fn (& [u8] , & 'a mut [u8]) -> Result < & 'a str , usize > ,) { let mut rng = rand :: thread_rng () ; let input = std :: iter :: repeat (()) . map (| () | rng . r#gen :: < u8 > ()) . take (len) . collect :: < Vec < _ > > () ; let mut dst = vec ! [0 ; input . len () * 2] ; b . bytes = len as u64 ; b . iter (| | unsafe { f (& input , & mut dst) . unwrap () ; dst [0] }) ; }
}

macro_rules! small_default_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function small_default in module {}", module_path!());
    };
}

mkfn!{
    small_default_introspect!();
    # [bench] fn small_default (b : & mut test :: Bencher) { doit (b , SMALL_LEN , hex_encode) ; }
}

macro_rules! small_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function small_fallback in module {}", module_path!());
    };
}

mkfn!{
    small_fallback_introspect!();
    # [bench] fn small_fallback (b : & mut test :: Bencher) { doit (b , SMALL_LEN , hex_encode_fallback) ; }
}

macro_rules! large_default_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function large_default in module {}", module_path!());
    };
}

mkfn!{
    large_default_introspect!();
    # [bench] fn large_default (b : & mut test :: Bencher) { doit (b , LARGE_LEN , hex_encode) ; }
}

macro_rules! large_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function large_fallback in module {}", module_path!());
    };
}

mkfn!{
    large_fallback_introspect!();
    # [bench] fn large_fallback (b : & mut test :: Bencher) { doit (b , LARGE_LEN , hex_encode_fallback) ; }
}
mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                mkuse!{use super :: * ;}

macro_rules! small_avx2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function small_avx2 in module {}", module_path!());
    };
}

mkfn!{
    small_avx2_introspect!();
    # [bench] fn small_avx2 (b : & mut test :: Bencher) { if self :: is_x86_feature_detected ! ("avx2") { doit (b , SMALL_LEN , hex_encode_avx2) ; } }
}

macro_rules! small_sse41_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function small_sse41 in module {}", module_path!());
    };
}

mkfn!{
    small_sse41_introspect!();
    # [bench] fn small_sse41 (b : & mut test :: Bencher) { if self :: is_x86_feature_detected ! ("sse4.1") { doit (b , SMALL_LEN , hex_encode_sse41) ; } }
}

macro_rules! large_avx2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function large_avx2 in module {}", module_path!());
    };
}

mkfn!{
    large_avx2_introspect!();
    # [bench] fn large_avx2 (b : & mut test :: Bencher) { if self :: is_x86_feature_detected ! ("avx2") { doit (b , LARGE_LEN , hex_encode_avx2) ; } }
}

macro_rules! large_sse41_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function large_sse41 in module {}", module_path!());
    };
}

mkfn!{
    large_sse41_introspect!();
    # [bench] fn large_sse41 (b : & mut test :: Bencher) { if self :: is_x86_feature_detected ! ("sse4.1") { doit (b , LARGE_LEN , hex_encode_sse41) ; } }
} 
            }} 
            }}