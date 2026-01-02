
macro_rules! truncf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncf16 in module {}", module_path!());
    };
}

mkfn!{
    truncf16_introspect!();
    # [doc = " Rounds the number toward 0 to the closest integral value (f16)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf16 (x : f16) -> f16 { super :: generic :: trunc (x) }
}

macro_rules! truncf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncf in module {}", module_path!());
    };
}

mkfn!{
    truncf_introspect!();
    # [doc = " Rounds the number toward 0 to the closest integral value (f32)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf (x : f32) -> f32 { select_implementation ! { name : truncf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: trunc (x) }
}

macro_rules! trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trunc in module {}", module_path!());
    };
}

mkfn!{
    trunc_introspect!();
    # [doc = " Rounds the number toward 0 to the closest integral value (f64)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn trunc (x : f64) -> f64 { select_implementation ! { name : trunc , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: trunc (x) }
}

macro_rules! truncf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncf128 in module {}", module_path!());
    };
}

mkfn!{
    truncf128_introspect!();
    # [doc = " Rounds the number toward 0 to the closest integral value (f128)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf128 (x : f128) -> f128 { super :: generic :: trunc (x) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                
macro_rules! sanity_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sanity_check in module {}", module_path!());
    };
}

mkfn!{
    sanity_check_introspect!();
    # [test] fn sanity_check () { assert_eq ! (super :: truncf (1.1) , 1.0) ; }
} 
            }}