mkuse!{use crate :: support :: { Float , FpResult , Int , IntTy , MinInt , Status } ;}

macro_rules! ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceil in module {}", module_path!());
    };
}

mkfn!{
    ceil_introspect!();
    # [inline] pub fn ceil < F : Float > (x : F) -> F { ceil_status (x) . val }
}

macro_rules! ceil_status_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceil_status in module {}", module_path!());
    };
}

mkfn!{
    ceil_status_introspect!();
    # [inline] pub fn ceil_status < F : Float > (x : F) -> FpResult < F > { let zero = IntTy :: < F > :: ZERO ; let mut ix = x . to_bits () ; let e = x . exp_unbiased () ; if e >= F :: SIG_BITS as i32 { return FpResult :: ok (x) ; } let status ; let res = if e >= 0 { let m = F :: SIG_MASK >> e . unsigned () ; if (ix & m) == zero { return FpResult :: ok (x) ; } status = Status :: INEXACT ; if x . is_sign_positive () { ix += m ; } ix &= ! m ; F :: from_bits (ix) } else { if ix & F :: SIG_MASK == F :: Int :: ZERO { status = Status :: OK ; } else { status = Status :: INEXACT ; } if x . is_sign_negative () { F :: NEG_ZERO } else if ix << 1 != zero { F :: ONE } else { x } } ; FpResult :: new (res , status) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: Hexf ;}

macro_rules! spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test in module {}", module_path!());
    };
}

mkfn!{
    spec_test_introspect!();
    # [doc = " Test against https://en.cppreference.com/w/cpp/numeric/math/ceil"] fn spec_test < F : Float > (cases : & [(F , F , Status)]) { let roundtrip = [F :: ZERO , F :: ONE , F :: NEG_ONE , F :: NEG_ZERO , F :: INFINITY , F :: NEG_INFINITY ,] ; for x in roundtrip { let FpResult { val , status } = ceil_status (x) ; assert_biteq ! (val , x , "{}" , Hexf (x)) ; assert_eq ! (status , Status :: OK , "{}" , Hexf (x)) ; } for & (x , res , res_stat) in cases { let FpResult { val , status } = ceil_status (x) ; assert_biteq ! (val , res , "{}" , Hexf (x)) ; assert_eq ! (status , res_stat , "{}" , Hexf (x)) ; } }
}

macro_rules! spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn spec_tests_f16 () { let cases = [(0.1 , 1.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 1.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 2.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 2.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f16 > (& cases) ; }
}

macro_rules! sanity_check_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f32 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f32_introspect!();
    # [test] fn sanity_check_f32 () { assert_eq ! (ceil (1.1f32) , 2.0) ; assert_eq ! (ceil (2.9f32) , 3.0) ; }
}

macro_rules! spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f32_introspect!();
    # [test] fn spec_tests_f32 () { let cases = [(0.1 , 1.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 1.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 2.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 2.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f32 > (& cases) ; }
}

macro_rules! sanity_check_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f64 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f64_introspect!();
    # [test] fn sanity_check_f64 () { assert_eq ! (ceil (1.1f64) , 2.0) ; assert_eq ! (ceil (2.9f64) , 3.0) ; }
}

macro_rules! spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f64_introspect!();
    # [test] fn spec_tests_f64 () { let cases = [(0.1 , 1.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 1.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 2.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 2.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f64 > (& cases) ; }
}

macro_rules! spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_tests_f128 () { let cases = [(0.1 , 1.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 1.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 2.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 2.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f128 > (& cases) ; }
} 
            }}