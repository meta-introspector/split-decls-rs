mkuse!{use super :: support :: { Float , Round } ;}

macro_rules! roundevenf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundevenf16 in module {}", module_path!());
    };
}

mkfn!{
    roundevenf16_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf16 (x : f16) -> f16 { roundeven_impl (x) }
}

macro_rules! roundevenf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundevenf in module {}", module_path!());
    };
}

mkfn!{
    roundevenf_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf (x : f32) -> f32 { roundeven_impl (x) }
}

macro_rules! roundeven_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundeven in module {}", module_path!());
    };
}

mkfn!{
    roundeven_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundeven (x : f64) -> f64 { roundeven_impl (x) }
}

macro_rules! roundevenf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundevenf128 in module {}", module_path!());
    };
}

mkfn!{
    roundevenf128_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf128 (x : f128) -> f128 { roundeven_impl (x) }
}

macro_rules! roundeven_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundeven_impl in module {}", module_path!());
    };
}

mkfn!{
    roundeven_impl_introspect!();
    # [inline] pub fn roundeven_impl < F : Float > (x : F) -> F { super :: generic :: rint_round (x , Round :: Nearest) . val }
}