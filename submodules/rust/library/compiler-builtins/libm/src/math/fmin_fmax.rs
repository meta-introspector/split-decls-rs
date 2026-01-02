
macro_rules! fminf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf16 in module {}", module_path!());
    };
}

mkfn!{
    fminf16_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmin (x , y) }
}

macro_rules! fminf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf in module {}", module_path!());
    };
}

mkfn!{
    fminf_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf (x : f32 , y : f32) -> f32 { super :: generic :: fmin (x , y) }
}

macro_rules! fmin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin in module {}", module_path!());
    };
}

mkfn!{
    fmin_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmin (x : f64 , y : f64) -> f64 { super :: generic :: fmin (x , y) }
}

macro_rules! fminf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf128 in module {}", module_path!());
    };
}

mkfn!{
    fminf128_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmin (x , y) }
}

macro_rules! fmaxf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf16 in module {}", module_path!());
    };
}

mkfn!{
    fmaxf16_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmax (x , y) }
}

macro_rules! fmaxf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf in module {}", module_path!());
    };
}

mkfn!{
    fmaxf_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf (x : f32 , y : f32) -> f32 { super :: generic :: fmax (x , y) }
}

macro_rules! fmax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax in module {}", module_path!());
    };
}

mkfn!{
    fmax_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmax (x : f64 , y : f64) -> f64 { super :: generic :: fmax (x , y) }
}

macro_rules! fmaxf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf128 in module {}", module_path!());
    };
}

mkfn!{
    fmaxf128_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmax (x , y) }
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

macro_rules! fmin_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_test_introspect!();
    fn fmin_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ZERO) , (F :: ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: ZERO , F :: INFINITY , F :: ZERO) , (F :: ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ZERO , F :: INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ZERO) , (F :: ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: ONE , F :: INFINITY , F :: ONE) , (F :: ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: ZERO) , (F :: INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: INFINITY , F :: ONE , F :: ONE) , (F :: INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fmin({}, {})" , Hexf (x) , Hexf (y)) ; } assert_eq ! (f (F :: ZERO , F :: NEG_ZERO) , F :: ZERO) ; assert_eq ! (f (F :: NEG_ZERO , F :: ZERO) , F :: ZERO) ; assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmin_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmin_spec_tests_f16 () { fmin_spec_test :: < f16 > (fminf16) ; }
}

macro_rules! fmin_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f32_introspect!();
    # [test] fn fmin_spec_tests_f32 () { fmin_spec_test :: < f32 > (fminf) ; }
}

macro_rules! fmin_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f64_introspect!();
    # [test] fn fmin_spec_tests_f64 () { fmin_spec_test :: < f64 > (fmin) ; }
}

macro_rules! fmin_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmin_spec_tests_f128 () { fmin_spec_test :: < f128 > (fminf128) ; }
}

macro_rules! fmax_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_test_introspect!();
    fn fmax_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ONE) , (F :: ZERO , F :: NEG_ONE , F :: ZERO) , (F :: ZERO , F :: INFINITY , F :: INFINITY) , (F :: ZERO , F :: NEG_INFINITY , F :: ZERO) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: ONE) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: INFINITY , F :: INFINITY) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ONE) , (F :: ONE , F :: NEG_ZERO , F :: ONE) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: ONE) , (F :: ONE , F :: INFINITY , F :: INFINITY) , (F :: ONE , F :: NEG_INFINITY , F :: ONE) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: ZERO) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ONE , F :: ONE , F :: ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: INFINITY) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: INFINITY) , (F :: INFINITY , F :: NEG_ZERO , F :: INFINITY) , (F :: INFINITY , F :: ONE , F :: INFINITY) , (F :: INFINITY , F :: NEG_ONE , F :: INFINITY) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: ZERO) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_INFINITY , F :: ONE , F :: ONE) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_INFINITY , F :: INFINITY , F :: INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fmax({}, {})" , Hexf (x) , Hexf (y)) ; } assert_eq ! (f (F :: ZERO , F :: NEG_ZERO) , F :: ZERO) ; assert_eq ! (f (F :: NEG_ZERO , F :: ZERO) , F :: ZERO) ; assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmax_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmax_spec_tests_f16 () { fmax_spec_test :: < f16 > (fmaxf16) ; }
}

macro_rules! fmax_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f32_introspect!();
    # [test] fn fmax_spec_tests_f32 () { fmax_spec_test :: < f32 > (fmaxf) ; }
}

macro_rules! fmax_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f64_introspect!();
    # [test] fn fmax_spec_tests_f64 () { fmax_spec_test :: < f64 > (fmax) ; }
}

macro_rules! fmax_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmax_spec_tests_f128 () { fmax_spec_test :: < f128 > (fmaxf128) ; }
} 
            }}