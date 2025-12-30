// Generated macro for impl_184 (impl)
macro_rules! Depcrate_exponentialimpl_184 {
() => {
// Module: crate::exponential
// Provides: {"impl_184"}
// Dependencies: {}
impl Distribution < f64 > for Exp1 { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> f64 { # [inline] fn pdf (x : f64) -> f64 { (- x) . exp () } # [inline] fn zero_case < R : Rng + ? Sized > (rng : & mut R , _u : f64) -> f64 { ziggurat_tables :: ZIG_EXP_R - rng . random :: < f64 > () . ln () } ziggurat (rng , false , & ziggurat_tables :: ZIG_EXP_X , & ziggurat_tables :: ZIG_EXP_F , pdf , zero_case ,) } }
};
}
