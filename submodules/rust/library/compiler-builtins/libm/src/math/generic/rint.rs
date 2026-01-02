mkuse!{use crate :: support :: { Float , FpResult , Round } ;}

macro_rules! rint_round_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rint_round in module {}", module_path!());
    };
}

mkfn!{
    rint_round_introspect!();
    # [doc = " IEEE 754-2019 `roundToIntegralExact`, which respects rounding mode and raises inexact if"] # [doc = " applicable."] # [inline] pub fn rint_round < F : Float > (x : F , _round : Round) -> FpResult < F > { let toint = F :: ONE / F :: EPSILON ; let e = x . ex () ; let positive = x . is_sign_positive () ; let force = | x | { if cfg ! (x86_no_sse) && (F :: BITS == 32 || F :: BITS == 64) { force_eval ! (x) } else { x } } ; let res = if e >= F :: EXP_BIAS + F :: SIG_BITS { x } else { let y = if positive { force (force (x) + toint) - toint } else { force (force (x) - toint) + toint } ; if y == F :: ZERO { if positive { F :: ZERO } else { F :: NEG_ZERO } } else { y } } ; FpResult :: ok (res) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: { Hexf , Status } ;}

macro_rules! spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test in module {}", module_path!());
    };
}

mkfn!{
    spec_test_introspect!();
    fn spec_test < F : Float > (cases : & [(F , F , Status)]) { let roundtrip = [F :: ZERO , F :: ONE , F :: NEG_ONE , F :: NEG_ZERO , F :: INFINITY , F :: NEG_INFINITY ,] ; for x in roundtrip { let FpResult { val , status } = rint_round (x , Round :: Nearest) ; assert_biteq ! (val , x , "rint_round({})" , Hexf (x)) ; assert_eq ! (status , Status :: OK , "{}" , Hexf (x)) ; } for & (x , res , res_stat) in cases { let FpResult { val , status } = rint_round (x , Round :: Nearest) ; assert_biteq ! (val , res , "rint_round({})" , Hexf (x)) ; assert_eq ! (status , res_stat , "{}" , Hexf (x)) ; } }
}

macro_rules! spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn spec_tests_f16 () { let cases = [] ; spec_test :: < f16 > (& cases) ; }
}

macro_rules! spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f32_introspect!();
    # [test] fn spec_tests_f32 () { let cases = [(0.1 , 0.0 , Status :: OK) , (- 0.1 , - 0.0 , Status :: OK) , (0.5 , 0.0 , Status :: OK) , (- 0.5 , - 0.0 , Status :: OK) , (0.9 , 1.0 , Status :: OK) , (- 0.9 , - 1.0 , Status :: OK) , (1.1 , 1.0 , Status :: OK) , (- 1.1 , - 1.0 , Status :: OK) , (1.5 , 2.0 , Status :: OK) , (- 1.5 , - 2.0 , Status :: OK) , (1.9 , 2.0 , Status :: OK) , (- 1.9 , - 2.0 , Status :: OK) , (2.8 , 3.0 , Status :: OK) , (- 2.8 , - 3.0 , Status :: OK) ,] ; spec_test :: < f32 > (& cases) ; }
}

macro_rules! spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f64_introspect!();
    # [test] fn spec_tests_f64 () { let cases = [(0.1 , 0.0 , Status :: OK) , (- 0.1 , - 0.0 , Status :: OK) , (0.5 , 0.0 , Status :: OK) , (- 0.5 , - 0.0 , Status :: OK) , (0.9 , 1.0 , Status :: OK) , (- 0.9 , - 1.0 , Status :: OK) , (1.1 , 1.0 , Status :: OK) , (- 1.1 , - 1.0 , Status :: OK) , (1.5 , 2.0 , Status :: OK) , (- 1.5 , - 2.0 , Status :: OK) , (1.9 , 2.0 , Status :: OK) , (- 1.9 , - 2.0 , Status :: OK) , (2.8 , 3.0 , Status :: OK) , (- 2.8 , - 3.0 , Status :: OK) ,] ; spec_test :: < f64 > (& cases) ; }
}

macro_rules! spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_tests_f128 () { let cases = [] ; spec_test :: < f128 > (& cases) ; }
} 
            }}