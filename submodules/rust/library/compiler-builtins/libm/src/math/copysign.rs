
macro_rules! copysignf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysignf16 in module {}", module_path!());
    };
}

mkfn!{
    copysignf16_introspect!();
    # [doc = " Sign of Y, magnitude of X (f16)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf16 (x : f16 , y : f16) -> f16 { super :: generic :: copysign (x , y) }
}

macro_rules! copysignf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysignf in module {}", module_path!());
    };
}

mkfn!{
    copysignf_introspect!();
    # [doc = " Sign of Y, magnitude of X (f32)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf (x : f32 , y : f32) -> f32 { super :: generic :: copysign (x , y) }
}

macro_rules! copysign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysign in module {}", module_path!());
    };
}

mkfn!{
    copysign_introspect!();
    # [doc = " Sign of Y, magnitude of X (f64)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysign (x : f64 , y : f64) -> f64 { super :: generic :: copysign (x , y) }
}

macro_rules! copysignf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysignf128 in module {}", module_path!());
    };
}

mkfn!{
    copysignf128_introspect!();
    # [doc = " Sign of Y, magnitude of X (f128)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf128 (x : f128 , y : f128) -> f128 { super :: generic :: copysign (x , y) }
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
    fn spec_test < F : Float > (f : impl Fn (F , F) -> F) { assert_biteq ! (f (F :: ZERO , F :: ZERO) , F :: ZERO) ; assert_biteq ! (f (F :: NEG_ZERO , F :: ZERO) , F :: ZERO) ; assert_biteq ! (f (F :: ZERO , F :: NEG_ZERO) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: NEG_ZERO , F :: NEG_ZERO) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: ONE , F :: ONE) , F :: ONE) ; assert_biteq ! (f (F :: NEG_ONE , F :: ONE) , F :: ONE) ; assert_biteq ! (f (F :: ONE , F :: NEG_ONE) , F :: NEG_ONE) ; assert_biteq ! (f (F :: NEG_ONE , F :: NEG_ONE) , F :: NEG_ONE) ; assert_biteq ! (f (F :: INFINITY , F :: INFINITY) , F :: INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY , F :: INFINITY) , F :: INFINITY) ; assert_biteq ! (f (F :: INFINITY , F :: NEG_INFINITY) , F :: NEG_INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY , F :: NEG_INFINITY) , F :: NEG_INFINITY) ; assert_biteq ! (f (F :: NAN , F :: NAN) , F :: NAN) ; assert_biteq ! (f (F :: NAN , F :: ONE) , F :: NAN) ; assert_biteq ! (f (F :: NAN , F :: NEG_ONE) , F :: NEG_NAN) ; assert_biteq ! (f (F :: NAN , F :: NEG_NAN) , F :: NEG_NAN) ; assert_biteq ! (f (F :: NEG_NAN , F :: NAN) , F :: NAN) ; assert_biteq ! (f (F :: NEG_NAN , F :: ONE) , F :: NAN) ; assert_biteq ! (f (F :: NEG_NAN , F :: NEG_ONE) , F :: NEG_NAN) ; assert_biteq ! (f (F :: NEG_NAN , F :: NEG_NAN) , F :: NEG_NAN) ; assert_biteq ! (f (F :: ONE , F :: NAN) , F :: ONE) ; assert_biteq ! (f (F :: ONE , F :: NEG_NAN) , F :: NEG_ONE) ; assert_biteq ! (f (F :: NEG_ONE , F :: NAN) , F :: ONE) ; assert_biteq ! (f (F :: NEG_ONE , F :: NEG_NAN) , F :: NEG_ONE) ; }
}

macro_rules! spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn spec_tests_f16 () { spec_test :: < f16 > (copysignf16) ; }
}

macro_rules! spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f32_introspect!();
    # [test] fn spec_tests_f32 () { spec_test :: < f32 > (copysignf) ; }
}

macro_rules! spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f64_introspect!();
    # [test] fn spec_tests_f64 () { spec_test :: < f64 > (copysign) ; }
}

macro_rules! spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_tests_f128 () { spec_test :: < f128 > (copysignf128) ; }
} 
            }}