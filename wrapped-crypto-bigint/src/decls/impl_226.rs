macro_rules! deps {
    () => {
        Uint!();
        Odd!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < const LIMBS : usize > Odd < Uint < LIMBS > > { # [doc = " Create a new [`Odd<Uint<LIMBS>>`] from the provided big endian hex string."] # [doc = ""] # [doc = " Panics if the hex is malformed or not zero-padded accordingly for the size, or if the value is even."] pub const fn from_be_hex (hex : & str) -> Self { let uint = Uint :: < LIMBS > :: from_be_hex (hex) ; assert ! (uint . is_odd () . is_true_vartime () , "number must be odd") ; Odd (uint) } # [doc = " Create a new [`Odd<Uint<LIMBS>>`] from the provided little endian hex string."] # [doc = ""] # [doc = " Panics if the hex is malformed or not zero-padded accordingly for the size, or if the value is even."] pub const fn from_le_hex (hex : & str) -> Self { let uint = Uint :: < LIMBS > :: from_be_hex (hex) ; assert ! (uint . is_odd () . is_true_vartime () , "number must be odd") ; Odd (uint) } # [doc = " Borrow the limbs of this [`Odd<Uint>`] as a [`Odd<UintRef>`]."] pub (crate) const fn as_uint_ref (& self) -> & Odd < UintRef > { # [allow (trivial_casts , unsafe_code)] unsafe { & * (self . 0 . as_uint_ref () as * const UintRef as * const Odd < UintRef >) } } # [doc = " Construct an [`Odd<Uint<T>>`] from the unsigned integer value,"] # [doc = " truncating the upper bits if the value is too large to be"] # [doc = " represented."] pub const fn resize < const T : usize > (& self) -> Odd < Uint < T > > { Odd (self . 0 . resize ()) } }
    };
}

impl_226!();