macro_rules! deps {
    () => {
        Rng!();
        Distribution!();
        StandardUniform!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [doc = " Note that on some hardware like x86/64 mask operations like [`_mm_blendv_epi8`]"] # [doc = " only care about a single bit. This means that you could use uniform random bits"] # [doc = " directly:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " // this may be faster..."] # [doc = " let x = unsafe { _mm_blendv_epi8(a.into(), b.into(), rng.random::<__m128i>()) };"] # [doc = ""] # [doc = " // ...than this"] # [doc = " let x = rng.random::<mask8x16>().select(b, a);"] # [doc = " ```"] # [doc = ""] # [doc = " Since most bits are unused you could also generate only as many bits as you need, i.e.:"] # [doc = " ```"] # [doc = " #![feature(portable_simd)]"] # [doc = " use std::simd::prelude::*;"] # [doc = " use rand::prelude::*;"] # [doc = " let mut rng = rand::rng();"] # [doc = ""] # [doc = " let x = u16x8::splat(rng.random::<u8>() as u16);"] # [doc = " let mask = u16x8::splat(1) << u16x8::from([0, 1, 2, 3, 4, 5, 6, 7]);"] # [doc = " let rand_mask = (x & mask).simd_eq(mask);"] # [doc = " ```"] # [doc = ""] # [doc = " [`_mm_blendv_epi8`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_blendv_epi8&ig_expand=514/"] # [doc = " [`simd_support`]: https://github.com/rust-random/rand#crate-features"] # [cfg (feature = "simd_support")] impl < T , const LANES : usize > Distribution < Mask < T , LANES > > for StandardUniform where T : MaskElement + Default , LaneCount < LANES > : SupportedLaneCount , StandardUniform : Distribution < Simd < T , LANES > > , Simd < T , LANES > : SimdPartialOrd < Mask = Mask < T , LANES > > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Mask < T , LANES > { let var = rng . random :: < Simd < T , LANES > > () ; var . simd_lt (Simd :: default ()) } }
    };
}

impl_151!();