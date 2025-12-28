macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < C > AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { # [doc = " Generate a cryptographically random [`AffinePoint`]."] pub fn try_from_rng < R : TryRngCore + ? Sized > (rng : & mut R ,) -> core :: result :: Result < Self , R :: Error > { let mut bytes = FieldBytes :: < C > :: default () ; let mut sign = 0 ; loop { rng . try_fill_bytes (& mut bytes) ? ; rng . try_fill_bytes (core :: array :: from_mut (& mut sign)) ? ; if let Some (point) = Self :: decompress (& bytes , Choice :: from (sign & 1)) . into_option () { return Ok (point) ; } } } }
    };
}

impl_17!();