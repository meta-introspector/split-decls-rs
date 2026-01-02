
macro_rules! scalbnf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scalbnf16 in module {}", module_path!());
    };
}

mkfn!{
    scalbnf16_introspect!();
    # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn scalbnf16 (x : f16 , n : i32) -> f16 { super :: generic :: scalbn (x , n) }
}

macro_rules! scalbnf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scalbnf in module {}", module_path!());
    };
}

mkfn!{
    scalbnf_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn scalbnf (x : f32 , n : i32) -> f32 { super :: generic :: scalbn (x , n) }
}

macro_rules! scalbn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scalbn in module {}", module_path!());
    };
}

mkfn!{
    scalbn_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn scalbn (x : f64 , n : i32) -> f64 { super :: generic :: scalbn (x , n) }
}

macro_rules! scalbnf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scalbnf128 in module {}", module_path!());
    };
}

mkfn!{
    scalbnf128_introspect!();
    # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn scalbnf128 (x : f128 , n : i32) -> f128 { super :: generic :: scalbn (x , n) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: { CastFrom , CastInto , Float } ;}

macro_rules! spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test in module {}", module_path!());
    };
}

mkfn!{
    spec_test_introspect!();
    fn spec_test < F : Float > (f : impl Fn (F , i32) -> F) where u32 : CastInto < F :: Int > , F :: Int : CastFrom < i32 > , F :: Int : CastFrom < u32 > , { assert_biteq ! (f (F :: NEG_ZERO , 10) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: NEG_ZERO , 0) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: NEG_ZERO , - 10) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: ZERO , 10) , F :: ZERO) ; assert_biteq ! (f (F :: ZERO , 0) , F :: ZERO) ; assert_biteq ! (f (F :: ZERO , - 10) , F :: ZERO) ; assert_biteq ! (f (F :: MIN , 0) , F :: MIN) ; assert_biteq ! (f (F :: MAX , 0) , F :: MAX) ; assert_biteq ! (f (F :: INFINITY , 0) , F :: INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY , 0) , F :: NEG_INFINITY) ; assert_biteq ! (f (F :: ZERO , 0) , F :: ZERO) ; assert_biteq ! (f (F :: NEG_ZERO , 0) , F :: NEG_ZERO) ; assert_biteq ! (f (F :: INFINITY , 10) , F :: INFINITY) ; assert_biteq ! (f (F :: INFINITY , - 10) , F :: INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY , 10) , F :: NEG_INFINITY) ; assert_biteq ! (f (F :: NEG_INFINITY , - 10) , F :: NEG_INFINITY) ; assert ! (f (F :: NAN , 10) . is_nan ()) ; assert ! (f (F :: NAN , 0) . is_nan ()) ; assert ! (f (F :: NAN , - 10) . is_nan ()) ; assert ! (f (- F :: NAN , 10) . is_nan ()) ; assert ! (f (- F :: NAN , 0) . is_nan ()) ; assert ! (f (- F :: NAN , - 10) . is_nan ()) ; }
}

macro_rules! spec_test_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f16 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn spec_test_f16 () { spec_test :: < f16 > (scalbnf16) ; }
}

macro_rules! spec_test_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f32_introspect!();
    # [test] fn spec_test_f32 () { spec_test :: < f32 > (scalbnf) ; }
}

macro_rules! spec_test_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f64_introspect!();
    # [test] fn spec_test_f64 () { spec_test :: < f64 > (scalbn) ; }
}

macro_rules! spec_test_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_test_f128 () { spec_test :: < f128 > (scalbnf128) ; }
} 
            }}