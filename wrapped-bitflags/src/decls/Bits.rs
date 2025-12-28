macro_rules! Bits {
    () => {
        # [doc = "\nA bits type that can be used as storage for a flags type.\n"] pub trait Bits : Clone + Copy + PartialEq + BitAnd < Output = Self > + BitOr < Output = Self > + BitXor < Output = Self > + Not < Output = Self > + Sized + 'static { # [doc = " A value with all bits unset."] const EMPTY : Self ; # [doc = " A value with all bits set."] const ALL : Self ; }
    };
}

Bits!()