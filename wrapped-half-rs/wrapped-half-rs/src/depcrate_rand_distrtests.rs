// Generated macro for tests (module)
macro_rules! Depcrate_rand_distrtests {
() => {
// Module: crate::rand_distr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [allow (unused_imports)] use rand :: { rng , Rng } ; use rand_distr :: { StandardNormal , StandardUniform , Uniform } ; # [test] fn test_sample_f16 () { let mut rng = rng () ; let _ : f16 = rng . sample (StandardUniform) ; let _ : f16 = rng . sample (StandardNormal) ; let _ : f16 = rng . sample (Uniform :: new (f16 :: from_f32 (0.0) , f16 :: from_f32 (1.0)) . unwrap ()) ; # [cfg (feature = "num-traits")] let _ : f16 = rng . sample (rand_distr :: Normal :: new (f16 :: from_f32 (0.0) , f16 :: from_f32 (1.0)) . unwrap ()) ; } # [test] fn test_sample_bf16 () { let mut rng = rng () ; let _ : bf16 = rng . sample (StandardUniform) ; let _ : bf16 = rng . sample (StandardNormal) ; let _ : bf16 = rng . sample (Uniform :: new (bf16 :: from_f32 (0.0) , bf16 :: from_f32 (1.0)) . unwrap ()) ; # [cfg (feature = "num-traits")] let _ : bf16 = rng . sample (rand_distr :: Normal :: new (bf16 :: from_f32 (0.0) , bf16 :: from_f32 (1.0)) . unwrap ()) ; } }
};
}
