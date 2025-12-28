macro_rules! deps {
    () => {
        CastInto!();
    };
}

macro_rules! cast_into_float {
    () => {
        deps!();
        macro_rules ! cast_into_float { ($ ty : ty) => { # [cfg (f16_enabled)] cast_into_float ! ($ ty ; f16) ; cast_into_float ! ($ ty ; f32 , f64) ; # [cfg (f128_enabled)] cast_into_float ! ($ ty ; f128) ; } ; ($ ty : ty ; $ ($ into : ty) ,*) => { $ (impl CastInto <$ into > for $ ty { fn cast (self) -> $ into { # [cfg (not (feature = "compiler-builtins"))] debug_assert_eq ! (self as $ into as $ ty , self , "inexact float cast") ; self as $ into } fn cast_lossy (self) -> $ into { self as $ into } }) * } ; }
    };
}

cast_into_float!()