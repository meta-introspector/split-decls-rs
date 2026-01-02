mkuse!{use crate :: support :: { Float , FpResult , Int , IntTy , MinInt , Status } ;}

macro_rules! trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trunc in module {}", module_path!());
    };
}

mkfn!{
    trunc_introspect!();
    # [inline] pub fn trunc < F : Float > (x : F) -> F { trunc_status (x) . val }
}

macro_rules! trunc_status_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trunc_status in module {}", module_path!());
    };
}

mkfn!{
    trunc_status_introspect!();
    # [inline] pub fn trunc_status < F : Float > (x : F) -> FpResult < F > { let mut xi : F :: Int = x . to_bits () ; let e : i32 = x . exp_unbiased () ; if e >= F :: SIG_BITS as i32 { return FpResult :: ok (x) ; } let mask = if e < 0 { F :: SIGN_MASK } else { ! (F :: SIG_MASK >> e . unsigned ()) } ; if (xi & ! mask) == IntTy :: < F > :: ZERO { return FpResult :: ok (x) ; } let status = if xi & F :: SIG_MASK == F :: Int :: ZERO { Status :: OK } else { Status :: INEXACT } ; xi &= mask ; FpResult :: new (F :: from_bits (xi) , status) }
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
    fn spec_test < F : Float > (cases : & [(F , F , Status)]) { let roundtrip = [F :: ZERO , F :: ONE , F :: NEG_ONE , F :: NEG_ZERO , F :: INFINITY , F :: NEG_INFINITY ,] ; for x in roundtrip { let FpResult { val , status } = trunc_status (x) ; assert_biteq ! (val , x , "{}" , Hexf (x)) ; assert_eq ! (status , Status :: OK , "{}" , Hexf (x)) ; } for & (x , res , res_stat) in cases { let FpResult { val , status } = trunc_status (x) ; assert_biteq ! (val , res , "{}" , Hexf (x)) ; assert_eq ! (status , res_stat , "{}" , Hexf (x)) ; } }
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

macro_rules! sanity_check_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f32 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f32_introspect!();
    # [test] fn sanity_check_f32 () { assert_eq ! (trunc (0.5f32) , 0.0) ; assert_eq ! (trunc (1.1f32) , 1.0) ; assert_eq ! (trunc (2.9f32) , 2.0) ; }
}

macro_rules! spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f32_introspect!();
    # [test] fn spec_tests_f32 () { let cases = [(0.1 , 0.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 0.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 1.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 1.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f32 > (& cases) ; assert_biteq ! (trunc (1.1f32) , 1.0) ; assert_biteq ! (trunc (1.1f64) , 1.0) ; assert_biteq ! (trunc (hf32 ! ("0x1p23")) , hf32 ! ("0x1p23")) ; assert_biteq ! (trunc (hf64 ! ("0x1p52")) , hf64 ! ("0x1p52")) ; assert_biteq ! (trunc (hf32 ! ("-0x1p23")) , hf32 ! ("-0x1p23")) ; assert_biteq ! (trunc (hf64 ! ("-0x1p52")) , hf64 ! ("-0x1p52")) ; assert_biteq ! (trunc (hf32 ! ("0x1p-1")) , 0.0) ; assert_biteq ! (trunc (hf64 ! ("0x1p-1")) , 0.0) ; assert_biteq ! (trunc (hf32 ! ("-0x1p-1")) , - 0.0) ; assert_biteq ! (trunc (hf64 ! ("-0x1p-1")) , - 0.0) ; }
}

macro_rules! sanity_check_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f64 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f64_introspect!();
    # [test] fn sanity_check_f64 () { assert_eq ! (trunc (1.1f64) , 1.0) ; assert_eq ! (trunc (2.9f64) , 2.0) ; }
}

macro_rules! spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_tests_f64_introspect!();
    # [test] fn spec_tests_f64 () { let cases = [(0.1 , 0.0 , Status :: INEXACT) , (- 0.1 , - 0.0 , Status :: INEXACT) , (0.9 , 0.0 , Status :: INEXACT) , (- 0.9 , - 0.0 , Status :: INEXACT) , (1.1 , 1.0 , Status :: INEXACT) , (- 1.1 , - 1.0 , Status :: INEXACT) , (1.9 , 1.0 , Status :: INEXACT) , (- 1.9 , - 1.0 , Status :: INEXACT) ,] ; spec_test :: < f64 > (& cases) ; }
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