macro_rules! deps {
    () => {
        MinInt!();
        DInt!();
        Int!();
    };
}

macro_rules! HInt {
    () => {
        deps!();
        # [doc = " Trait for integers half the bit width of another integer. This is implemented for all"] # [doc = " primitives except for `u128`, because it there is not a larger primitive."] pub trait HInt : Int { # [doc = " Integer that is double the bit width of the integer this trait is implemented for"] type D : DInt < H = Self > + MinInt ; # [doc = " Widens (using default extension) the integer to have double bit width"] fn widen (self) -> Self :: D ; # [doc = " Widens (zero extension only) the integer to have double bit width. This is needed to get"] # [doc = " around problems with associated type bounds (such as `Int<Othersign: DInt>`) being unstable"] fn zero_widen (self) -> Self :: D ; # [doc = " Widens the integer to have double bit width and shifts the integer into the higher bits"] # [allow (unused)] fn widen_hi (self) -> Self :: D ; # [doc = " Widening multiplication with zero widening. This cannot overflow."] fn zero_widen_mul (self , rhs : Self) -> Self :: D ; # [doc = " Widening multiplication. This cannot overflow."] fn widen_mul (self , rhs : Self) -> Self :: D ; }
    };
}

HInt!();