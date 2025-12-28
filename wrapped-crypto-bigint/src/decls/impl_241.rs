macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Odd < BoxedUint > { # [doc = " Borrow the limbs of this [`Odd<BoxedUint>`] as a [`Odd<UintRef>`]."] pub (crate) const fn as_uint_ref (& self) -> & Odd < UintRef > { # [allow (trivial_casts , unsafe_code)] unsafe { & * (self . 0 . as_uint_ref () as * const UintRef as * const Odd < UintRef >) } } }
    };
}

impl_241!();