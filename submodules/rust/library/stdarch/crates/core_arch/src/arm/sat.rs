mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! __ssat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __ssat in module {}", module_path!());
    };
}

mkfn!{
    __ssat_introspect!();
    # [doc = " Saturates a 32-bit signed integer to a signed integer with a given"] # [doc = " bit width."] # [unstable (feature = "stdarch_arm_sat" , issue = "none")] # [inline] # [cfg_attr (test , assert_instr ("ssat" , WIDTH = 8))] # [rustc_legacy_const_generics (1)] pub unsafe fn __ssat < const WIDTH : u32 > (x : i32) -> i32 { static_assert ! (matches ! (WIDTH , 1 ..= 32)) ; arm_ssat (x , WIDTH as i32) }
}

macro_rules! __usat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __usat in module {}", module_path!());
    };
}

mkfn!{
    __usat_introspect!();
    # [doc = " Saturates a 32-bit signed integer to an unsigned integer with a given"] # [doc = " bit width."] # [unstable (feature = "stdarch_arm_sat" , issue = "none")] # [inline] # [cfg_attr (test , assert_instr ("usat" , WIDTH = 8))] # [rustc_legacy_const_generics (1)] pub unsafe fn __usat < const WIDTH : u32 > (x : i32) -> u32 { static_assert ! (matches ! (WIDTH , 1 ..= 32)) ; arm_usat (x , WIDTH as i32) }
}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.arm.ssat"] fn arm_ssat (x : i32 , y : i32) -> i32 ; # [link_name = "llvm.arm.usat"] fn arm_usat (x : i32 , y : i32) -> u32 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_ssat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ssat in module {}", module_path!());
    };
}

mkfn!{
    test_ssat_introspect!();
    # [test] fn test_ssat () { unsafe { assert_eq ! (__ssat ::< 8 > (1) , 1) ; assert_eq ! (__ssat ::< 8 > (1000) , 127) ; assert_eq ! (__ssat ::< 8 > (- 1) , - 1) ; assert_eq ! (__ssat ::< 8 > (- 1000) , - 128) ; } }
}

macro_rules! test_usat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_usat in module {}", module_path!());
    };
}

mkfn!{
    test_usat_introspect!();
    # [test] fn test_usat () { unsafe { assert_eq ! (__usat ::< 8 > (1) , 1) ; assert_eq ! (__usat ::< 8 > (1000) , 255) ; assert_eq ! (__usat ::< 8 > (- 1) , 0) ; assert_eq ! (__usat ::< 8 > (- 1000) , 0) ; } }
} 
            }}