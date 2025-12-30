// Generated macro for Signed (trait)
macro_rules! Depcrate_traitsSigned {
() => {
// Module: crate::traits
// Provides: {"Signed"}
// Dependencies: {}
# [doc = " Signed [`Integer`]s."] pub trait Signed : Div < NonZero < Self > , Output = CtOption < Self > > + for < 'a > Div < & 'a NonZero < Self > , Output = CtOption < Self > > + From < i8 > + From < i16 > + From < i32 > + From < i64 > + Integer { # [doc = " Corresponding unsigned integer type."] type Unsigned : Unsigned ; # [doc = " The sign and magnitude of this [`Signed`]."] fn abs_sign (& self) -> (Self :: Unsigned , Choice) ; # [doc = " The magnitude of this [`Signed`]."] fn abs (& self) -> Self :: Unsigned { self . abs_sign () . 0 } # [doc = " Whether this [`Signed`] is negative (and non-zero), as a [`Choice`]."] fn is_negative (& self) -> Choice ; # [doc = " Whether this [`Signed`] is positive (and non-zero), as a [`Choice`]."] fn is_positive (& self) -> Choice ; }
};
}
