macro_rules! deps {
    () => {
        IntoFloat!();
        Distribution!();
        OpenClosed01!();
        Rng!();
        Open01!();
        StandardUniform!();
    };
}

macro_rules! float_impls {
    () => {
        deps!();
        macro_rules ! float_impls { ($ ($ meta : meta) ?, $ ty : ident , $ uty : ident , $ f_scalar : ident , $ u_scalar : ty , $ fraction_bits : expr , $ exponent_bias : expr) => { $ (# [cfg ($ meta)]) ? impl IntoFloat for $ uty { type F = $ ty ; # [inline (always)] fn into_float_with_exponent (self , exponent : i32) -> $ ty { let exponent_bits : $ u_scalar = (($ exponent_bias + exponent) as $ u_scalar) << $ fraction_bits ; $ ty :: from_bits (self | $ uty :: splat (exponent_bits)) } } $ (# [cfg ($ meta)]) ? impl Distribution <$ ty > for StandardUniform { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { let float_size = mem :: size_of ::<$ f_scalar > () as $ u_scalar * 8 ; let precision = $ fraction_bits + 1 ; let scale = 1.0 / ((1 as $ u_scalar << precision) as $ f_scalar) ; let value : $ uty = rng . random () ; let value = value >> $ uty :: splat (float_size - precision) ; $ ty :: splat (scale) * $ ty :: cast_from_int (value) } } $ (# [cfg ($ meta)]) ? impl Distribution <$ ty > for OpenClosed01 { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { let float_size = mem :: size_of ::<$ f_scalar > () as $ u_scalar * 8 ; let precision = $ fraction_bits + 1 ; let scale = 1.0 / ((1 as $ u_scalar << precision) as $ f_scalar) ; let value : $ uty = rng . random () ; let value = value >> $ uty :: splat (float_size - precision) ; $ ty :: splat (scale) * $ ty :: cast_from_int (value + $ uty :: splat (1)) } } $ (# [cfg ($ meta)]) ? impl Distribution <$ ty > for Open01 { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { let float_size = mem :: size_of ::<$ f_scalar > () as $ u_scalar * 8 ; let value : $ uty = rng . random () ; let fraction = value >> $ uty :: splat (float_size - $ fraction_bits) ; fraction . into_float_with_exponent (0) - $ ty :: splat (1.0 - $ f_scalar :: EPSILON / 2.0) } } } }
    };
}

float_impls!();