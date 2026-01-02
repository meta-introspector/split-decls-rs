mkuse!{use super :: { copysign , trunc } ;}
mkuse!{use crate :: support :: { Float , MinInt } ;}

macro_rules! round_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round in module {}", module_path!());
    };
}

mkfn!{
    round_introspect!();
    # [inline] pub fn round < F : Float > (x : F) -> F { let f0p5 = F :: from_parts (false , F :: EXP_BIAS - 1 , F :: Int :: ZERO) ; let f0p25 = F :: from_parts (false , F :: EXP_BIAS - 2 , F :: Int :: ZERO) ; trunc (x + copysign (f0p5 - f0p25 * F :: EPSILON , x)) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! zeroes_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroes_f16 in module {}", module_path!());
    };
}

mkfn!{
    zeroes_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn zeroes_f16 () { assert_biteq ! (round (0.0_f16) , 0.0_f16) ; assert_biteq ! (round (- 0.0_f16) , - 0.0_f16) ; }
}

macro_rules! sanity_check_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f16 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn sanity_check_f16 () { assert_eq ! (round (- 1.0_f16) , - 1.0) ; assert_eq ! (round (2.8_f16) , 3.0) ; assert_eq ! (round (- 0.5_f16) , - 1.0) ; assert_eq ! (round (0.5_f16) , 1.0) ; assert_eq ! (round (- 1.5_f16) , - 2.0) ; assert_eq ! (round (1.5_f16) , 2.0) ; }
}

macro_rules! zeroes_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroes_f32 in module {}", module_path!());
    };
}

mkfn!{
    zeroes_f32_introspect!();
    # [test] fn zeroes_f32 () { assert_biteq ! (round (0.0_f32) , 0.0_f32) ; assert_biteq ! (round (- 0.0_f32) , - 0.0_f32) ; }
}

macro_rules! sanity_check_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f32 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f32_introspect!();
    # [test] fn sanity_check_f32 () { assert_eq ! (round (- 1.0_f32) , - 1.0) ; assert_eq ! (round (2.8_f32) , 3.0) ; assert_eq ! (round (- 0.5_f32) , - 1.0) ; assert_eq ! (round (0.5_f32) , 1.0) ; assert_eq ! (round (- 1.5_f32) , - 2.0) ; assert_eq ! (round (1.5_f32) , 2.0) ; }
}

macro_rules! zeroes_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroes_f64 in module {}", module_path!());
    };
}

mkfn!{
    zeroes_f64_introspect!();
    # [test] fn zeroes_f64 () { assert_biteq ! (round (0.0_f64) , 0.0_f64) ; assert_biteq ! (round (- 0.0_f64) , - 0.0_f64) ; }
}

macro_rules! sanity_check_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f64 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f64_introspect!();
    # [test] fn sanity_check_f64 () { assert_eq ! (round (- 1.0_f64) , - 1.0) ; assert_eq ! (round (2.8_f64) , 3.0) ; assert_eq ! (round (- 0.5_f64) , - 1.0) ; assert_eq ! (round (0.5_f64) , 1.0) ; assert_eq ! (round (- 1.5_f64) , - 2.0) ; assert_eq ! (round (1.5_f64) , 2.0) ; }
}

macro_rules! zeroes_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroes_f128 in module {}", module_path!());
    };
}

mkfn!{
    zeroes_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn zeroes_f128 () { assert_biteq ! (round (0.0_f128) , 0.0_f128) ; assert_biteq ! (round (- 0.0_f128) , - 0.0_f128) ; }
}

macro_rules! sanity_check_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check_f128 in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn sanity_check_f128 () { assert_eq ! (round (- 1.0_f128) , - 1.0) ; assert_eq ! (round (2.8_f128) , 3.0) ; assert_eq ! (round (- 0.5_f128) , - 1.0) ; assert_eq ! (round (0.5_f128) , 1.0) ; assert_eq ! (round (- 1.5_f128) , - 2.0) ; assert_eq ! (round (1.5_f128) , 2.0) ; }
} 
            }}