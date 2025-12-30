// Generated macro for impl_215 (impl)
macro_rules! Depcrate_frechetimpl_215 {
() => {
// Module: crate::frechet
// Provides: {"impl_215"}
// Dependencies: {}
impl < F > Distribution < F > for Frechet < F > where F : Float , OpenClosed01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let x : F = rng . sample (OpenClosed01) ; self . location + self . scale * (- x . ln ()) . powf (- self . shape . recip ()) } }
};
}
