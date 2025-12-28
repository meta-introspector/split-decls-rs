macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_common {
    () => {
        deps!();
        macro_rules ! impl_common { ($ ty : ty) => { impl ops :: BitOr for $ ty { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self :: Output { self . lo |= rhs . lo ; self . hi |= rhs . hi ; self } } impl ops :: Not for $ ty { type Output = Self ; fn not (mut self) -> Self :: Output { self . lo = ! self . lo ; self . hi = ! self . hi ; self } } impl ops :: Add < Self > for $ ty { type Output = Self ; fn add (self , rhs : Self) -> Self :: Output { let (lo , carry) = self . lo . overflowing_add (rhs . lo) ; let (hi , of) = Int :: carrying_add (self . hi , rhs . hi , carry) ; debug_assert ! (! of , "attempt to add with overflow") ; Self { lo , hi } } } impl ops :: Sub < Self > for $ ty { type Output = Self ; fn sub (self , rhs : Self) -> Self :: Output { let (lo , borrow) = self . lo . overflowing_sub (rhs . lo) ; let (hi , of) = Int :: borrowing_sub (self . hi , rhs . hi , borrow) ; debug_assert ! (! of , "attempt to subtract with overflow") ; Self { lo , hi } } } impl ops :: Shl < u32 > for $ ty { type Output = Self ; fn shl (mut self , rhs : u32) -> Self :: Output { debug_assert ! (rhs < Self :: BITS , "attempt to shift left with overflow") ; let half_bits = Self :: BITS / 2 ; let low_mask = half_bits - 1 ; let s = rhs & low_mask ; let lo = self . lo ; let hi = self . hi ; self . lo = lo << s ; if rhs & half_bits == 0 { self . hi = (lo >> (low_mask ^ s) >> 1) as _ ; self . hi |= hi << s ; } else { self . hi = self . lo as _ ; self . lo = 0 ; } self } } impl ops :: Shr < u32 > for $ ty { type Output = Self ; fn shr (mut self , rhs : u32) -> Self :: Output { debug_assert ! (rhs < Self :: BITS , "attempt to shift right with overflow") ; let half_bits = Self :: BITS / 2 ; let low_mask = half_bits - 1 ; let s = rhs & low_mask ; let lo = self . lo ; let hi = self . hi ; self . hi = hi >> s ; # [allow (unused_comparisons)] if rhs & half_bits == 0 { self . lo = (hi << (low_mask ^ s) << 1) as _ ; self . lo |= lo >> s ; } else { self . lo = self . hi as _ ; self . hi = if hi < 0 { ! 0 } else { 0 } ; } self } } } ; }
    };
}

impl_common!()