macro_rules! deps {
    () => {
        FiniteBitSet!();
    };
}

macro_rules! FiniteBitSetTy {
    () => {
        deps!();
        # [doc = " Integral type used to represent the bit set."] pub trait FiniteBitSetTy : BitAnd < Output = Self > + BitAndAssign + BitOrAssign + Clone + Copy + Shl + Not < Output = Self > + PartialEq + Sized { # [doc = " Size of the domain representable by this type, e.g. 64 for `u64`."] const DOMAIN_SIZE : u32 ; # [doc = " Value which represents the `FiniteBitSet` having every bit set."] const FILLED : Self ; # [doc = " Value which represents the `FiniteBitSet` having no bits set."] const EMPTY : Self ; # [doc = " Value for one as the integral type."] const ONE : Self ; # [doc = " Value for zero as the integral type."] const ZERO : Self ; # [doc = " Perform a checked left shift on the integral type."] fn checked_shl (self , rhs : u32) -> Option < Self > ; # [doc = " Perform a checked right shift on the integral type."] fn checked_shr (self , rhs : u32) -> Option < Self > ; }
    };
}

FiniteBitSetTy!()