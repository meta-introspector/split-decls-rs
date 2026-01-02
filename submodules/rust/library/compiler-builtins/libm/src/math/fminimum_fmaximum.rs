
macro_rules! fminimumf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimumf16 in module {}", module_path!());
    };
}

mkfn!{
    fminimumf16_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf16 (x : f16 , y : f16) -> f16 { super :: generic :: fminimum (x , y) }
}

macro_rules! fminimum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum in module {}", module_path!());
    };
}

mkfn!{
    fminimum_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum (x : f64 , y : f64) -> f64 { super :: generic :: fminimum (x , y) }
}

macro_rules! fminimumf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimumf in module {}", module_path!());
    };
}

mkfn!{
    fminimumf_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf (x : f32 , y : f32) -> f32 { super :: generic :: fminimum (x , y) }
}

macro_rules! fminimumf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimumf128 in module {}", module_path!());
    };
}

mkfn!{
    fminimumf128_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf128 (x : f128 , y : f128) -> f128 { super :: generic :: fminimum (x , y) }
}

macro_rules! fmaximumf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximumf16 in module {}", module_path!());
    };
}

mkfn!{
    fmaximumf16_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmaximum (x , y) }
}

macro_rules! fmaximumf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximumf in module {}", module_path!());
    };
}

mkfn!{
    fmaximumf_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf (x : f32 , y : f32) -> f32 { super :: generic :: fmaximum (x , y) }
}

macro_rules! fmaximum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum (x : f64 , y : f64) -> f64 { super :: generic :: fmaximum (x , y) }
}

macro_rules! fmaximumf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximumf128 in module {}", module_path!());
    };
}

mkfn!{
    fmaximumf128_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmaximum (x , y) }
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

macro_rules! fminimum_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fminimum_spec_test_introspect!();
    fn fminimum_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ZERO , F :: ONE , F :: ZERO) , (F :: ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: ZERO , F :: INFINITY , F :: ZERO) , (F :: ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ZERO , F :: NAN , F :: NAN) , (F :: NEG_ZERO , F :: ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ZERO , F :: INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ZERO , F :: NAN , F :: NAN) , (F :: ONE , F :: ZERO , F :: ZERO) , (F :: ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: ONE , F :: INFINITY , F :: ONE) , (F :: ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ONE , F :: NAN , F :: NAN) , (F :: NEG_ONE , F :: ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ONE , F :: NAN , F :: NAN) , (F :: INFINITY , F :: ZERO , F :: ZERO) , (F :: INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: INFINITY , F :: ONE , F :: ONE) , (F :: INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: INFINITY , F :: NAN , F :: NAN) , (F :: NEG_INFINITY , F :: ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NAN) , (F :: NAN , F :: ZERO , F :: NAN) , (F :: NAN , F :: NEG_ZERO , F :: NAN) , (F :: NAN , F :: ONE , F :: NAN) , (F :: NAN , F :: NEG_ONE , F :: NAN) , (F :: NAN , F :: INFINITY , F :: NAN) , (F :: NAN , F :: NEG_INFINITY , F :: NAN) , (F :: NAN , F :: NAN , F :: NAN) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fminimum({}, {})" , Hexf (x) , Hexf (y)) ; } assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: ZERO , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_ZERO , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: ONE , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_ONE , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: INFINITY , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_INFINITY , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: ZERO) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_ZERO) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: ONE) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_ONE) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: INFINITY) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_INFINITY) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fminimum_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fminimum_spec_tests_f16 () { fminimum_spec_test :: < f16 > (fminimumf16) ; }
}

macro_rules! fminimum_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_spec_tests_f32_introspect!();
    # [test] fn fminimum_spec_tests_f32 () { fminimum_spec_test :: < f32 > (fminimumf) ; }
}

macro_rules! fminimum_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_spec_tests_f64_introspect!();
    # [test] fn fminimum_spec_tests_f64 () { fminimum_spec_test :: < f64 > (fminimum) ; }
}

macro_rules! fminimum_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fminimum_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fminimum_spec_tests_f128 () { fminimum_spec_test :: < f128 > (fminimumf128) ; }
}

macro_rules! fmaximum_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_spec_test_introspect!();
    fn fmaximum_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: NEG_ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ONE) , (F :: ZERO , F :: NEG_ONE , F :: ZERO) , (F :: ZERO , F :: INFINITY , F :: INFINITY) , (F :: ZERO , F :: NEG_INFINITY , F :: ZERO) , (F :: ZERO , F :: NAN , F :: NAN) , (F :: NEG_ZERO , F :: ZERO , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: ONE) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: INFINITY , F :: INFINITY) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NAN , F :: NAN) , (F :: ONE , F :: ZERO , F :: ONE) , (F :: ONE , F :: NEG_ZERO , F :: ONE) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: ONE) , (F :: ONE , F :: INFINITY , F :: INFINITY) , (F :: ONE , F :: NEG_INFINITY , F :: ONE) , (F :: ONE , F :: NAN , F :: NAN) , (F :: NEG_ONE , F :: ZERO , F :: ZERO) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ONE , F :: ONE , F :: ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: INFINITY) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NAN , F :: NAN) , (F :: INFINITY , F :: ZERO , F :: INFINITY) , (F :: INFINITY , F :: NEG_ZERO , F :: INFINITY) , (F :: INFINITY , F :: ONE , F :: INFINITY) , (F :: INFINITY , F :: NEG_ONE , F :: INFINITY) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NAN , F :: NAN) , (F :: NEG_INFINITY , F :: ZERO , F :: ZERO) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_INFINITY , F :: ONE , F :: ONE) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_INFINITY , F :: INFINITY , F :: INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NAN) , (F :: NAN , F :: ZERO , F :: NAN) , (F :: NAN , F :: NEG_ZERO , F :: NAN) , (F :: NAN , F :: ONE , F :: NAN) , (F :: NAN , F :: NEG_ONE , F :: NAN) , (F :: NAN , F :: INFINITY , F :: NAN) , (F :: NAN , F :: NEG_INFINITY , F :: NAN) , (F :: NAN , F :: NAN , F :: NAN) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fmaximum({}, {})" , Hexf (x) , Hexf (y)) ; } assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: ZERO , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_ZERO , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: ONE , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_ONE , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: INFINITY , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_INFINITY , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: ZERO) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_ZERO) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: ONE) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_ONE) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: INFINITY) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_INFINITY) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmaximum_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmaximum_spec_tests_f16 () { fmaximum_spec_test :: < f16 > (fmaximumf16) ; }
}

macro_rules! fmaximum_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_spec_tests_f32_introspect!();
    # [test] fn fmaximum_spec_tests_f32 () { fmaximum_spec_test :: < f32 > (fmaximumf) ; }
}

macro_rules! fmaximum_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_spec_tests_f64_introspect!();
    # [test] fn fmaximum_spec_tests_f64 () { fmaximum_spec_test :: < f64 > (fmaximum) ; }
}

macro_rules! fmaximum_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmaximum_spec_tests_f128 () { fmaximum_spec_test :: < f128 > (fmaximumf128) ; }
} 
            }}