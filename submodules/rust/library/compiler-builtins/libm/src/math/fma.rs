mkuse!{use super :: generic ;}
mkuse!{use crate :: support :: Round ;}

macro_rules! fmaf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf16 in module {}", module_path!());
    };
}

mkfn!{
    fmaf16_introspect!();
    # [allow (unused)] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn fmaf16 (_x : f16 , _y : f16 , _z : f16) -> f16 { unimplemented ! () }
}

macro_rules! fmaf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf in module {}", module_path!());
    };
}

mkfn!{
    fmaf_introspect!();
    # [doc = " Floating multiply add (f32)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaf (x : f32 , y : f32 , z : f32) -> f32 { select_implementation ! { name : fmaf , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , target_feature = "sse2" ,) , args : x , y , z , } generic :: fma_wide_round (x , y , z , Round :: Nearest) . val }
}

macro_rules! fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma in module {}", module_path!());
    };
}

mkfn!{
    fma_introspect!();
    # [doc = " Fused multiply add (f64)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fma (x : f64 , y : f64 , z : f64) -> f64 { select_implementation ! { name : fma , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , target_feature = "sse2" ,) , args : x , y , z , } generic :: fma_round (x , y , z , Round :: Nearest) . val }
}

macro_rules! fmaf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf128 in module {}", module_path!());
    };
}

mkfn!{
    fmaf128_introspect!();
    # [doc = " Fused multiply add (f128)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaf128 (x : f128 , y : f128 , z : f128) -> f128 { generic :: fma_round (x , y , z , Round :: Nearest) . val }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: { CastFrom , CastInto , Float , FpResult , HInt , MinInt , Round , Status } ;}

macro_rules! spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test in module {}", module_path!());
    };
}

mkfn!{
    spec_test_introspect!();
    # [doc = " Test the generic `fma_round` algorithm for a given float."] fn spec_test < F > (f : impl Fn (F , F , F) -> F) where F : Float , F : CastFrom < F :: SignedInt > , F : CastFrom < i8 > , F :: Int : HInt , u32 : CastInto < F :: Int > , { let x = F :: from_bits (F :: Int :: ONE) ; let y = F :: from_bits (F :: Int :: ONE) ; let z = F :: ZERO ; assert_biteq ! (f (x , y , z) , F :: ZERO) ; assert_biteq ! (f (x , - y , z) , F :: NEG_ZERO) ; assert_biteq ! (f (- x , y , z) , F :: NEG_ZERO) ; assert_biteq ! (f (- x , - y , z) , F :: ZERO) ; }
}

macro_rules! spec_test_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f32 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f32_introspect!();
    # [test] fn spec_test_f32 () { spec_test :: < f32 > (fmaf) ; spec_test :: < f32 > (| x , y , z | generic :: fma_round (x , y , z , Round :: Nearest) . val) ; }
}

macro_rules! spec_test_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f64 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f64_introspect!();
    # [test] fn spec_test_f64 () { spec_test :: < f64 > (fma) ; let expect_underflow = [(hf64 ! ("0x1.0p-1070") , hf64 ! ("0x1.0p-1070") , hf64 ! ("0x1.ffffffffffffp-1023") , hf64 ! ("0x0.ffffffffffff8p-1022") ,) , (hf64 ! ("0x1.0p-1070") , hf64 ! ("0x1.0p-1070") , hf64 ! ("-0x1.0p-1022") , hf64 ! ("-0x1.0p-1022") ,) ,] ; for (x , y , z , res) in expect_underflow { let FpResult { val , status } = generic :: fma_round (x , y , z , Round :: Nearest) ; assert_biteq ! (val , res) ; assert_eq ! (status , Status :: UNDERFLOW) ; } }
}

macro_rules! spec_test_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spec_test_f128 in module {}", module_path!());
    };
}

mkfn!{
    spec_test_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn spec_test_f128 () { spec_test :: < f128 > (fmaf128) ; }
}

macro_rules! issue_263_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function issue_263 in module {}", module_path!());
    };
}

mkfn!{
    issue_263_introspect!();
    # [test] fn issue_263 () { let a = f32 :: from_bits (1266679807) ; let b = f32 :: from_bits (1300234242) ; let c = f32 :: from_bits (1115553792) ; let expected = f32 :: from_bits (1501560833) ; assert_eq ! (fmaf (a , b , c) , expected) ; }
}

macro_rules! fma_segfault_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_segfault in module {}", module_path!());
    };
}

mkfn!{
    fma_segfault_introspect!();
    # [test] fn fma_segfault () { assert_eq ! (fma (- 0.0000000000000002220446049250313 , - 0.0000000000000002220446049250313 , - 0.0000000000000002220446049250313) , - 0.00000000000000022204460492503126 ,) ; let result = fma (- 0.992 , - 0.992 , - 0.992) ; # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] let result = force_eval ! (result) ; assert_eq ! (result , - 0.007936000000000007 ,) ; }
}

macro_rules! fma_sbb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_sbb in module {}", module_path!());
    };
}

mkfn!{
    fma_sbb_introspect!();
    # [test] fn fma_sbb () { assert_eq ! (fma (- (1.0 - f64 :: EPSILON) , f64 :: MIN , f64 :: MIN) , - 3991680619069439e277) ; }
}

macro_rules! fma_underflow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_underflow in module {}", module_path!());
    };
}

mkfn!{
    fma_underflow_introspect!();
    # [test] fn fma_underflow () { assert_eq ! (fma (1.1102230246251565e-16 , - 9.812526705433188e-305 , 1.0894e-320) , 0.0 ,) ; }
} 
            }}