mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _bswap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bswap in module {}", module_path!());
    };
}

mkfn!{
    _bswap_introspect!();
    # [doc = " Returns an integer with the reversed byte order of x"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_bswap)"] # [inline] # [cfg_attr (test , assert_instr (bswap))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _bswap (x : i32) -> i32 { x . swap_bytes () }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! test_bswap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bswap in module {}", module_path!());
    };
}

mkfn!{
    test_bswap_introspect!();
    # [test] fn test_bswap () { unsafe { assert_eq ! (_bswap (0x0EADBE0F) , 0x0FBEAD0E) ; assert_eq ! (_bswap (0x00000000) , 0x00000000) ; } }
} 
            }}