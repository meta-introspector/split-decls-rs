
macro_rules! fabsf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabsf16 in module {}", module_path!());
    };
}

mkfn!{
    fabsf16_introspect!();
    # [doc = " Absolute value (magnitude) (f16)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf16 (x : f16) -> f16 { super :: generic :: fabs (x) }
}

macro_rules! fabsf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabsf in module {}", module_path!());
    };
}

mkfn!{
    fabsf_introspect!();
    # [doc = " Absolute value (magnitude) (f32)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf (x : f32) -> f32 { select_implementation ! { name : fabsf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: fabs (x) }
}

macro_rules! fabs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabs in module {}", module_path!());
    };
}

mkfn!{
    fabs_introspect!();
    # [doc = " Absolute value (magnitude) (f64)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabs (x : f64) -> f64 { select_implementation ! { name : fabs , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: fabs (x) }
}

macro_rules! fabsf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabsf128 in module {}", module_path!());
    };
}

mkfn!{
    fabsf128_introspect!();
    # [doc = " Absolute value (magnitude) (f128)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf128 (x : f128) -> f128 { super :: generic :: fabs (x) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: Float ;}

macro_rules! spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test in module {}", module_path!());
    };
}

mkfn!{
    spec_test_introspect!();
    # [doc = " Based on https://en.cppreference.com/w/cpp/numeric/math/fabs"] fn spec_test < F : Float > (f : impl Fn (F) -> F) { assert_biteq ! (f (F :: ZERO) , F :: ZERO) ; assert_biteq ! (f (F :: NEG_ZERO) , F :: ZERO) ; assert_biteq ! (f (F :: INFINITY) , F :: INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY) , F :: INFINITY) ; assert ! (f (F :: NAN) . is_nan ()) ; assert ! (f (F :: NAN) . is_sign_positive ()) ; assert ! (f (F :: from_bits (F :: NAN . to_bits () | F :: SIGN_MASK)) . is_sign_positive ()) ; }
}

macro_rules! sanity_check_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f16 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn sanity_check_f16 () { assert_eq ! (fabsf16 (- 1.0f16) , 1.0) ; assert_eq ! (fabsf16 (2.8f16) , 2.8) ; }
}

macro_rules! spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn spec_tests_f16 () { spec_test :: < f16 > (fabsf16) ; }
}

macro_rules! sanity_check_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f32 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f32_introspect!();
    # [test] fn sanity_check_f32 () { assert_eq ! (fabsf (- 1.0f32) , 1.0) ; assert_eq ! (fabsf (2.8f32) , 2.8) ; }
}

macro_rules! spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f32_introspect!();
    # [test] fn spec_tests_f32 () { spec_test :: < f32 > (fabsf) ; }
}

macro_rules! sanity_check_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f64 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f64_introspect!();
    # [test] fn sanity_check_f64 () { assert_eq ! (fabs (- 1.0f64) , 1.0) ; assert_eq ! (fabs (2.8f64) , 2.8) ; }
}

macro_rules! spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f64_introspect!();
    # [test] fn spec_tests_f64 () { spec_test :: < f64 > (fabs) ; }
}

macro_rules! sanity_check_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f128 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn sanity_check_f128 () { assert_eq ! (fabsf128 (- 1.0f128) , 1.0) ; assert_eq ! (fabsf128 (2.8f128) , 2.8) ; }
}

macro_rules! spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_tests_f128 () { spec_test :: < f128 > (fabsf128) ; }
} 
            }}