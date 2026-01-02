mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! _cvtmask64_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _cvtmask64_u64 in module {}", module_path!());
    };
}

mkfn!{
    _cvtmask64_u64_introspect!();
    # [doc = " Convert 64-bit mask a into an integer value, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_cvtmask64_u64)"] # [inline] # [target_feature (enable = "avx512bw")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _cvtmask64_u64 (a : __mmask64) -> u64 { a }
}

macro_rules! _cvtu64_mask64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _cvtu64_mask64 in module {}", module_path!());
    };
}

mkfn!{
    _cvtu64_mask64_introspect!();
    # [doc = " Convert integer value a into an 64-bit mask, and store the result in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_cvtu64_mask64)"] # [inline] # [target_feature (enable = "avx512bw")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _cvtu64_mask64 (a : u64) -> __mmask64 { a }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: { x86 :: * , x86_64 :: * } ;}

macro_rules! test_cvtmask64_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_cvtmask64_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_cvtmask64_u64_introspect!();
    # [simd_test (enable = "avx512bw")] unsafe fn test_cvtmask64_u64 () { let a : __mmask64 = 0b11001100_00110011_01100110_10011001 ; let r = _cvtmask64_u64 (a) ; let e : u64 = 0b11001100_00110011_01100110_10011001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_cvtu64_mask64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_cvtu64_mask64 in module {}", module_path!());
    };
}

mkfn!{
    test_cvtu64_mask64_introspect!();
    # [simd_test (enable = "avx512bw")] unsafe fn test_cvtu64_mask64 () { let a : u64 = 0b11001100_00110011_01100110_10011001 ; let r = _cvtu64_mask64 (a) ; let e : __mmask64 = 0b11001100_00110011_01100110_10011001 ; assert_eq ! (r , e) ; }
} 
            }}