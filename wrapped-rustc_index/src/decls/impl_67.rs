macro_rules! deps {
    () => {
        FiniteBitSet!();
        FiniteBitSetTy!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T : FiniteBitSetTy > FiniteBitSet < T > { # [doc = " Creates a new, empty bitset."] pub fn new_empty () -> Self { Self (T :: EMPTY) } # [doc = " Sets the `index`th bit."] pub fn set (& mut self , index : u32) { self . 0 |= T :: ONE . checked_shl (index) . unwrap_or (T :: ZERO) ; } # [doc = " Unsets the `index`th bit."] pub fn clear (& mut self , index : u32) { self . 0 &= ! T :: ONE . checked_shl (index) . unwrap_or (T :: ZERO) ; } # [doc = " Sets the `i`th to `j`th bits."] pub fn set_range (& mut self , range : Range < u32 >) { let bits = T :: FILLED . checked_shl (range . end - range . start) . unwrap_or (T :: ZERO) . not () . checked_shl (range . start) . unwrap_or (T :: ZERO) ; self . 0 |= bits ; } # [doc = " Is the set empty?"] pub fn is_empty (& self) -> bool { self . 0 == T :: EMPTY } # [doc = " Returns the domain size of the bitset."] pub fn within_domain (& self , index : u32) -> bool { index < T :: DOMAIN_SIZE } # [doc = " Returns if the `index`th bit is set."] pub fn contains (& self , index : u32) -> Option < bool > { self . within_domain (index) . then (| | ((self . 0 . checked_shr (index) . unwrap_or (T :: ONE)) & T :: ONE) == T :: ONE) } }
    };
}

impl_67!()