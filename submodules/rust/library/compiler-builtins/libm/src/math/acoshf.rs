mkuse!{use super :: { log1pf , logf , sqrtf } ;}
mkitem!{const LN2 : f32 = 0.693147180559945309417232121458176568 ;}

macro_rules! acoshf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function acoshf in module {}", module_path!());
    };
}

mkfn!{
    acoshf_introspect!();
    # [doc = " Inverse hyperbolic cosine (f32)"] # [doc = ""] # [doc = " Calculates the inverse hyperbolic cosine of `x`."] # [doc = " Is defined as `log(x + sqrt(x*x-1))`."] # [doc = " `x` must be a number greater than or equal to 1."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn acoshf (x : f32) -> f32 { let u = x . to_bits () ; let a = u & 0x7fffffff ; if a < 0x3f800000 + (1 << 23) { return log1pf (x - 1.0 + sqrtf ((x - 1.0) * (x - 1.0) + 2.0 * (x - 1.0))) ; } if a < 0x3f800000 + (12 << 23) { return logf (2.0 * x - 1.0 / (x + sqrtf (x * x - 1.0))) ; } return logf (x) + LN2 ; }
}