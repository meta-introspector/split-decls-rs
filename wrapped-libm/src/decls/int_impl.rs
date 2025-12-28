macro_rules! deps {
    () => {
        MinInt!();
        Int!();
        OtherSign!();
    };
}

macro_rules! int_impl {
    () => {
        deps!();
        macro_rules ! int_impl { ($ ity : ty , $ uty : ty) => { impl MinInt for $ uty { type OtherSign = $ ity ; type Unsigned = $ uty ; const BITS : u32 = < Self as MinInt >:: ZERO . count_zeros () ; const SIGNED : bool = Self :: MIN != Self :: ZERO ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MIN : Self = < Self >:: MIN ; const MAX : Self = < Self >:: MAX ; } impl Int for $ uty { fn signed (self) -> $ ity { self as $ ity } fn unsigned (self) -> Self { self } fn abs (self) -> Self { unimplemented ! () } fn unsigned_abs (self) -> Self { unimplemented ! () } # [allow (clippy :: wrong_self_convention)] fn from_unsigned (me : $ uty) -> Self { me } fn abs_diff (self , other : Self) -> Self { self . abs_diff (other) } int_impl_common ! ($ uty) ; } impl MinInt for $ ity { type OtherSign = $ uty ; type Unsigned = $ uty ; const BITS : u32 = < Self as MinInt >:: ZERO . count_zeros () ; const SIGNED : bool = Self :: MIN != Self :: ZERO ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MIN : Self = < Self >:: MIN ; const MAX : Self = < Self >:: MAX ; } impl Int for $ ity { fn signed (self) -> Self { self } fn unsigned (self) -> $ uty { self as $ uty } fn abs (self) -> Self { self . abs () } fn unsigned_abs (self) -> Self :: Unsigned { self . unsigned_abs () } fn from_unsigned (me : $ uty) -> Self { me as $ ity } fn abs_diff (self , other : Self) -> $ uty { self . abs_diff (other) } int_impl_common ! ($ ity) ; } } ; }
    };
}

int_impl!();