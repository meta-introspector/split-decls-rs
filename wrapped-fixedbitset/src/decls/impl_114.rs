macro_rules! deps {
    () => {
        Ones!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Ones < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { while self . bitset_back == 0 { match self . remaining_blocks . next_back () { None => { if self . bitset_front != 0 { self . bitset_back = 0 ; self . block_idx_back = self . block_idx_front ; return Some (self . block_idx_front + BITS - Self :: first_positive_bit_and_unset (& mut self . bitset_front) - 1 ,) ; } else { return None ; } } Some (next_block) => { self . bitset_back = * next_block ; self . block_idx_back -= BITS ; } } ; } Some (self . block_idx_back - Self :: first_positive_bit_and_unset (& mut self . bitset_back) + BITS - 1 ,) } }
    };
}

impl_114!();