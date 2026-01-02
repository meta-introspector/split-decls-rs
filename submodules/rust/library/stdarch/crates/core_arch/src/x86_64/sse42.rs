mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.sse42.crc32.64.64"] fn crc32_64_64 (crc : u64 , v : u64) -> u64 ; }}

macro_rules! _mm_crc32_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_crc32_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_crc32_u64_introspect!();
    # [doc = " Starting with the initial value in `crc`, return the accumulated"] # [doc = " CRC32-C value for unsigned 64-bit integer `v`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_crc32_u64)"] # [inline] # [target_feature (enable = "sse4.2")] # [cfg_attr (test , assert_instr (crc32))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_crc32_u64 (crc : u64 , v : u64) -> u64 { unsafe { crc32_64_64 (crc , v) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: arch :: x86_64 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm_crc32_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_crc32_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_crc32_u64_introspect!();
    # [simd_test (enable = "sse4.2")] unsafe fn test_mm_crc32_u64 () { let crc = 0x7819dccd3e824 ; let v = 0x2a22b845fed ; let i = _mm_crc32_u64 (crc , v) ; assert_eq ! (i , 0xbb6cdc6c) ; }
} 
            }}