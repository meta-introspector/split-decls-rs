macro_rules! deps {
    () => {
        AndNot!();
    };
}

macro_rules! BitOps0 {
    () => {
        deps!();
        # [doc = " Ops that are independent of word size and endian"] pub trait BitOps0 : BitAnd < Output = Self > + BitOr < Output = Self > + BitXor < Output = Self > + BitXorAssign + Not < Output = Self > + AndNot < Output = Self > + Sized + Copy + Clone { }
    };
}

BitOps0!();