mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _bswap64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bswap64 in module {}", module_path!());
    };
}

mkfn!{
    _bswap64_introspect!();
    # [doc = " Returns an integer with the reversed byte order of x"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bswap64)"] # [inline] # [cfg_attr (test , assert_instr (bswap))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _bswap64 (x : i64) -> i64 { x . swap_bytes () }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! test_bswap64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bswap64 in module {}", module_path!());
    };
}

mkfn!{
    test_bswap64_introspect!();
    # [test] fn test_bswap64 () { unsafe { assert_eq ! (_bswap64 (0x0EADBEEFFADECA0E) , 0x0ECADEFAEFBEAD0E) ; assert_eq ! (_bswap64 (0x0000000000000000) , 0x0000000000000000) ; } }
} 
            }}