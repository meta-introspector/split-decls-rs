// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : Clone + Num > Complex < T > { # [doc = " Returns the imaginary unit."] # [doc = ""] # [doc = " See also [`Complex::I`]."] # [inline] pub fn i () -> Self { Self :: new (T :: zero () , T :: one ()) } # [doc = " Returns the square of the norm (since `T` doesn't necessarily"] # [doc = " have a sqrt function), i.e. `re^2 + im^2`."] # [inline] pub fn norm_sqr (& self) -> T { self . re . clone () * self . re . clone () + self . im . clone () * self . im . clone () } # [doc = " Multiplies `self` by the scalar `t`."] # [inline] pub fn scale (& self , t : T) -> Self { Self :: new (self . re . clone () * t . clone () , self . im . clone () * t) } # [doc = " Divides `self` by the scalar `t`."] # [inline] pub fn unscale (& self , t : T) -> Self { Self :: new (self . re . clone () / t . clone () , self . im . clone () / t) } # [doc = " Raises `self` to an unsigned integer power."] # [inline] pub fn powu (& self , exp : u32) -> Self { Pow :: pow (self , exp) } }
};
}
