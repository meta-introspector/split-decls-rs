// Generated macro for impl_268 (impl)
macro_rules! Depcrate_rand_distrimpl_268 {
() => {
// Module: crate::rand_distr
// Provides: {"impl_268"}
// Dependencies: {}
impl rand_distr :: uniform :: UniformSampler for Float16Sampler { type X = f16 ; fn new < B1 , B2 > (low : B1 , high : B2) -> Result < Self , rand_distr :: uniform :: Error > where B1 : rand_distr :: uniform :: SampleBorrow < Self :: X > + Sized , B2 : rand_distr :: uniform :: SampleBorrow < Self :: X > + Sized , { Ok (Self (UniformFloat :: new (low . borrow () . to_f32 () , high . borrow () . to_f32 () ,) ?)) } fn new_inclusive < B1 , B2 > (low : B1 , high : B2) -> Result < Self , rand_distr :: uniform :: Error > where B1 : rand_distr :: uniform :: SampleBorrow < Self :: X > + Sized , B2 : rand_distr :: uniform :: SampleBorrow < Self :: X > + Sized , { Ok (Self (UniformFloat :: new_inclusive (low . borrow () . to_f32 () , high . borrow () . to_f32 () ,) ?)) } fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Self :: X { f16 :: from_f32 (self . 0 . sample (rng)) } }
};
}
