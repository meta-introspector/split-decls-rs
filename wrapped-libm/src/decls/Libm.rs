macro_rules! Libm {
    () => {
        # [doc = " Generic helper for libm functions, abstracting over f32 and f64. <br/>"] # [doc = " # Type Parameter:"] # [doc = " - `T`: Either `f32` or `f64`"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust"] # [doc = " use libm::{self, Libm};"] # [doc = ""] # [doc = " const PI_F32: f32 = 3.1415927410e+00;"] # [doc = " const PI_F64: f64 = 3.1415926535897931160e+00;"] # [doc = ""] # [doc = " assert!(Libm::<f32>::cos(0.0f32) == libm::cosf(0.0));"] # [doc = " assert!(Libm::<f32>::sin(PI_F32) == libm::sinf(PI_F32));"] # [doc = ""] # [doc = " assert!(Libm::<f64>::cos(0.0f64) == libm::cos(0.0));"] # [doc = " assert!(Libm::<f64>::sin(PI_F64) == libm::sin(PI_F64));"] # [doc = " ```"] pub struct Libm < T > (PhantomData < T >) ;
    };
}

Libm!()