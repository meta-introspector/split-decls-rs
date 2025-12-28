macro_rules! deps {
    () => {
        FiniteBitSetTy!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl FiniteBitSetTy for u32 { const DOMAIN_SIZE : u32 = 32 ; const FILLED : Self = Self :: MAX ; const EMPTY : Self = Self :: MIN ; const ONE : Self = 1u32 ; const ZERO : Self = 0u32 ; fn checked_shl (self , rhs : u32) -> Option < Self > { self . checked_shl (rhs) } fn checked_shr (self , rhs : u32) -> Option < Self > { self . checked_shr (rhs) } }
    };
}

impl_64!();