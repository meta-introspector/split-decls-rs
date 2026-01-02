
macro_rules! fminimum_numf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_numf16 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_numf16_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf16 (x : f16 , y : f16) -> f16 { super :: generic :: fminimum_num (x , y) }
}

macro_rules! fminimum_numf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_numf in module {}", module_path!());
    };
}

mkfn!{
    fminimum_numf_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf (x : f32 , y : f32) -> f32 { super :: generic :: fminimum_num (x , y) }
}

macro_rules! fminimum_num_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_num (x : f64 , y : f64) -> f64 { super :: generic :: fminimum_num (x , y) }
}

macro_rules! fminimum_numf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_numf128 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_numf128_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf128 (x : f128 , y : f128) -> f128 { super :: generic :: fminimum_num (x , y) }
}

macro_rules! fmaximum_numf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_numf16 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_numf16_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_numf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmaximum_num (x , y) }
}

macro_rules! fmaximum_numf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_numf in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_numf_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_numf (x : f32 , y : f32) -> f32 { super :: generic :: fmaximum_num (x , y) }
}

macro_rules! fmaximum_num_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_num (x : f64 , y : f64) -> f64 { super :: generic :: fmaximum_num (x , y) }
}

macro_rules! fmaximum_numf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_numf128 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_numf128_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_numf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmaximum_num (x , y) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: { Float , Hexf } ;}

macro_rules! fminimum_num_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_spec_test_introspect!();
    fn fminimum_num_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ZERO , F :: ONE , F :: ZERO) , (F :: ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: ZERO , F :: INFINITY , F :: ZERO) , (F :: ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ZERO , F :: INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ZERO) , (F :: ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: ONE , F :: INFINITY , F :: ONE) , (F :: ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: ZERO) , (F :: INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: INFINITY , F :: ONE , F :: ONE) , (F :: INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , expected) in cases { let actual = f (x , y) ; assert_biteq ! (actual , expected , "fminimum_num({}, {})" , Hexf (x) , Hexf (y)) ; } assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fminimum_num_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fminimum_num_spec_tests_f16 () { fminimum_num_spec_test :: < f16 > (fminimum_numf16) ; }
}

macro_rules! fminimum_num_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_spec_tests_f32_introspect!();
    # [test] fn fminimum_num_spec_tests_f32 () { fminimum_num_spec_test :: < f32 > (fminimum_numf) ; }
}

macro_rules! fminimum_num_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_spec_tests_f64_introspect!();
    # [test] fn fminimum_num_spec_tests_f64 () { fminimum_num_spec_test :: < f64 > (fminimum_num) ; }
}

macro_rules! fminimum_num_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fminimum_num_spec_tests_f128 () { fminimum_num_spec_test :: < f128 > (fminimum_numf128) ; }
}

macro_rules! fmaximum_num_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_spec_test_introspect!();
    fn fmaximum_num_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: NEG_ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ONE) , (F :: ZERO , F :: NEG_ONE , F :: ZERO) , (F :: ZERO , F :: INFINITY , F :: INFINITY) , (F :: ZERO , F :: NEG_INFINITY , F :: ZERO) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: ZERO , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: ONE) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: INFINITY , F :: INFINITY) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ONE) , (F :: ONE , F :: NEG_ZERO , F :: ONE) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: ONE) , (F :: ONE , F :: INFINITY , F :: INFINITY) , (F :: ONE , F :: NEG_INFINITY , F :: ONE) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: ZERO) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ONE , F :: ONE , F :: ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: INFINITY) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: INFINITY) , (F :: INFINITY , F :: NEG_ZERO , F :: INFINITY) , (F :: INFINITY , F :: ONE , F :: INFINITY) , (F :: INFINITY , F :: NEG_ONE , F :: INFINITY) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: ZERO) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_INFINITY , F :: ONE , F :: ONE) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_INFINITY , F :: INFINITY , F :: INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , expected) in cases { let actual = f (x , y) ; assert_biteq ! (actual , expected , "fmaximum_num({}, {})" , Hexf (x) , Hexf (y)) ; } assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmaximum_num_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmaximum_num_spec_tests_f16 () { fmaximum_num_spec_test :: < f16 > (fmaximum_numf16) ; }
}

macro_rules! fmaximum_num_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_spec_tests_f32_introspect!();
    # [test] fn fmaximum_num_spec_tests_f32 () { fmaximum_num_spec_test :: < f32 > (fmaximum_numf) ; }
}

macro_rules! fmaximum_num_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_spec_tests_f64_introspect!();
    # [test] fn fmaximum_num_spec_tests_f64 () { fmaximum_num_spec_test :: < f64 > (fmaximum_num) ; }
}

macro_rules! fmaximum_num_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmaximum_num_spec_tests_f128 () { fmaximum_num_spec_test :: < f128 > (fmaximum_numf128) ; }
} 
            }}