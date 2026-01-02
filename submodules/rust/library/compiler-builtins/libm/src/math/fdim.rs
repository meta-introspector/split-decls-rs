
macro_rules! fdimf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdimf16 in module {}", module_path!());
    };
}

mkfn!{
    fdimf16_introspect!();
    # [doc = " Positive difference (f16)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf16 (x : f16 , y : f16) -> f16 { super :: generic :: fdim (x , y) }
}

macro_rules! fdimf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdimf in module {}", module_path!());
    };
}

mkfn!{
    fdimf_introspect!();
    # [doc = " Positive difference (f32)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf (x : f32 , y : f32) -> f32 { super :: generic :: fdim (x , y) }
}

macro_rules! fdim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdim in module {}", module_path!());
    };
}

mkfn!{
    fdim_introspect!();
    # [doc = " Positive difference (f64)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdim (x : f64 , y : f64) -> f64 { super :: generic :: fdim (x , y) }
}

macro_rules! fdimf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdimf128 in module {}", module_path!());
    };
}

mkfn!{
    fdimf128_introspect!();
    # [doc = " Positive difference (f128)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf128 (x : f128 , y : f128) -> f128 { super :: generic :: fdim (x , y) }
}