mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse4a.extrq"] fn extrq (x : i64x2 , y : i8x16) -> i64x2 ; # [link_name = "llvm.x86.sse4a.extrqi"] fn extrqi (x : i64x2 , len : u8 , idx : u8) -> i64x2 ; # [link_name = "llvm.x86.sse4a.insertq"] fn insertq (x : i64x2 , y : i64x2) -> i64x2 ; # [link_name = "llvm.x86.sse4a.insertqi"] fn insertqi (x : i64x2 , y : i64x2 , len : u8 , idx : u8) -> i64x2 ; # [link_name = "llvm.x86.sse4a.movnt.sd"] fn movntsd (x : * mut f64 , y : __m128d) ; # [link_name = "llvm.x86.sse4a.movnt.ss"] fn movntss (x : * mut f32 , y : __m128) ; }}

macro_rules! _mm_extract_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_extract_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_extract_si64_introspect!();
    # [doc = " Extracts the bit range specified by `y` from the lower 64 bits of `x`."] # [doc = ""] # [doc = " The `[13:8]` bits of `y` specify the index of the bit-range to extract. The"] # [doc = " `[5:0]` bits of `y` specify the length of the bit-range to extract. All"] # [doc = " other bits are ignored."] # [doc = ""] # [doc = " If the length is zero, it is interpreted as `64`. If the length and index"] # [doc = " are zero, the lower 64 bits of `x` are extracted."] # [doc = ""] # [doc = " If `length == 0 && index > 0` or `length + index > 64` the result is"] # [doc = " undefined."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (extrq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_extract_si64 (x : __m128i , y : __m128i) -> __m128i { unsafe { transmute (extrq (x . as_i64x2 () , y . as_i8x16 ())) } }
}

macro_rules! _mm_extracti_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_extracti_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_extracti_si64_introspect!();
    # [doc = " Extracts the specified bits from the lower 64 bits of the 128-bit integer vector operand at the"] # [doc = " index `idx` and of the length `len`."] # [doc = ""] # [doc = " `idx` specifies the index of the LSB. `len` specifies the number of bits to extract. If length"] # [doc = " and index are both zero, bits `[63:0]` of parameter `x` are extracted. It is a compile-time error"] # [doc = " for `len + idx` to be greater than 64 or for `len` to be zero and `idx` to be non-zero."] # [doc = ""] # [doc = " Returns a 128-bit integer vector whose lower 64 bits contain the extracted bits."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (extrq , LEN = 5 , IDX = 5))] # [rustc_legacy_const_generics (1 , 2)] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub fn _mm_extracti_si64 < const LEN : i32 , const IDX : i32 > (x : __m128i) -> __m128i { static_assert_uimm_bits ! (LEN , 6) ; static_assert_uimm_bits ! (IDX , 6) ; static_assert ! ((LEN == 0 && IDX == 0) || (LEN != 0 && LEN + IDX <= 64)) ; unsafe { transmute (extrqi (x . as_i64x2 () , LEN as u8 , IDX as u8)) } }
}

macro_rules! _mm_insert_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_insert_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_insert_si64_introspect!();
    # [doc = " Inserts the `[length:0]` bits of `y` into `x` at `index`."] # [doc = ""] # [doc = " The bits of `y`:"] # [doc = ""] # [doc = " - `[69:64]` specify the `length`,"] # [doc = " - `[77:72]` specify the index."] # [doc = ""] # [doc = " If the `length` is zero it is interpreted as `64`. If `index + length > 64`"] # [doc = " or `index > 0 && length == 0` the result is undefined."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (insertq))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_insert_si64 (x : __m128i , y : __m128i) -> __m128i { unsafe { transmute (insertq (x . as_i64x2 () , y . as_i64x2 ())) } }
}

macro_rules! _mm_inserti_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_inserti_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_inserti_si64_introspect!();
    # [doc = " Inserts the `len` least-significant bits from the lower 64 bits of the 128-bit integer vector operand `y` into"] # [doc = " the lower 64 bits of the 128-bit integer vector operand `x` at the index `idx` and of the length `len`."] # [doc = ""] # [doc = " `idx` specifies the index of the LSB. `len` specifies the number of bits to insert. If length and index"] # [doc = " are both zero, bits `[63:0]` of parameter `x` are replaced with bits `[63:0]` of parameter `y`. It is a"] # [doc = " compile-time error for `len + idx` to be greater than 64 or for `len` to be zero and `idx` to be non-zero."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (insertq , LEN = 5 , IDX = 5))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "simd_x86_updates" , since = "1.82.0")] pub fn _mm_inserti_si64 < const LEN : i32 , const IDX : i32 > (x : __m128i , y : __m128i) -> __m128i { static_assert_uimm_bits ! (LEN , 6) ; static_assert_uimm_bits ! (IDX , 6) ; static_assert ! ((LEN == 0 && IDX == 0) || (LEN != 0 && LEN + IDX <= 64)) ; unsafe { transmute (insertqi (x . as_i64x2 () , y . as_i64x2 () , LEN as u8 , IDX as u8)) } }
}

macro_rules! _mm_stream_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_sd_introspect!();
    # [doc = " Non-temporal store of `a.0` into `p`."] # [doc = ""] # [doc = " Writes 64-bit data to a memory location without polluting the caches."] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (movntsd))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_stream_sd (p : * mut f64 , a : __m128d) { movntsd (p , a) ; }
}

macro_rules! _mm_stream_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_stream_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_stream_ss_introspect!();
    # [doc = " Non-temporal store of `a.0` into `p`."] # [doc = ""] # [doc = " Writes 32-bit data to a memory location without polluting the caches."] # [doc = ""] # [doc = " # Safety of non-temporal stores"] # [doc = ""] # [doc = " After using this intrinsic, but before any other access to the memory that this intrinsic"] # [doc = " mutates, a call to [`_mm_sfence`] must be performed by the thread that used the intrinsic. In"] # [doc = " particular, functions that call this intrinsic should generally call `_mm_sfence` before they"] # [doc = " return."] # [doc = ""] # [doc = " See [`_mm_sfence`] for details."] # [inline] # [target_feature (enable = "sse4a")] # [cfg_attr (test , assert_instr (movntss))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _mm_stream_ss (p : * mut f32 , a : __m128) { movntss (p , a) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm_extract_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_extract_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_extract_si64_introspect!();
    # [simd_test (enable = "sse4a")] unsafe fn test_mm_extract_si64 () { let b = 0b0110_0000_0000_i64 ; let x = _mm_setr_epi64x (b , 0) ; let v = 0b001000___00___000100_i64 ; let y = _mm_setr_epi64x (v , 0) ; let e = _mm_setr_epi64x (0b0110_i64 , 0) ; let r = _mm_extract_si64 (x , y) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_extracti_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_extracti_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_extracti_si64_introspect!();
    # [simd_test (enable = "sse4a")] unsafe fn test_mm_extracti_si64 () { let a = _mm_setr_epi64x (0x0123456789abcdef , 0) ; let r = _mm_extracti_si64 :: < 8 , 8 > (a) ; let e = _mm_setr_epi64x (0xcd , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_insert_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_insert_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_insert_si64_introspect!();
    # [simd_test (enable = "sse4a")] unsafe fn test_mm_insert_si64 () { let i = 0b0110_i64 ; let z = 0b1010_1010_1010i64 ; let e = 0b0110_1010_1010i64 ; let x = _mm_setr_epi64x (z , 0) ; let expected = _mm_setr_epi64x (e , 0) ; let v = 0b001000___00___000100_i64 ; let y = _mm_setr_epi64x (i , v) ; let r = _mm_insert_si64 (x , y) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_inserti_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_inserti_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_inserti_si64_introspect!();
    # [simd_test (enable = "sse4a")] unsafe fn test_mm_inserti_si64 () { let a = _mm_setr_epi64x (0x0123456789abcdef , 0) ; let b = _mm_setr_epi64x (0x0011223344556677 , 0) ; let r = _mm_inserti_si64 :: < 8 , 8 > (a , b) ; let e = _mm_setr_epi64x (0x0123456789ab77ef , 0) ; assert_eq_m128i (r , e) ; }
}
mkitem!{mkstruct!{# [repr (align (16))] struct MemoryF64 { data : [f64 ; 2] , }}}

macro_rules! test_mm_stream_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_sd_introspect!();
    # [simd_test (enable = "sse4a")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_sd () { let mut mem = MemoryF64 { data : [1.0_f64 , 2.0] , } ; { let vals = & mut mem . data ; let d = vals . as_mut_ptr () ; let x = _mm_setr_pd (3.0 , 4.0) ; _mm_stream_sd (d , x) ; } assert_eq ! (mem . data [0] , 3.0) ; assert_eq ! (mem . data [1] , 2.0) ; }
}
mkitem!{mkstruct!{# [repr (align (16))] struct MemoryF32 { data : [f32 ; 4] , }}}

macro_rules! test_mm_stream_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_stream_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_stream_ss_introspect!();
    # [simd_test (enable = "sse4a")] # [cfg_attr (miri , ignore)] unsafe fn test_mm_stream_ss () { let mut mem = MemoryF32 { data : [1.0_f32 , 2.0 , 3.0 , 4.0] , } ; { let vals = & mut mem . data ; let d = vals . as_mut_ptr () ; let x = _mm_setr_ps (5.0 , 6.0 , 7.0 , 8.0) ; _mm_stream_ss (d , x) ; } assert_eq ! (mem . data [0] , 5.0) ; assert_eq ! (mem . data [1] , 2.0) ; assert_eq ! (mem . data [2] , 3.0) ; assert_eq ! (mem . data [3] , 4.0) ; }
} 
            }}