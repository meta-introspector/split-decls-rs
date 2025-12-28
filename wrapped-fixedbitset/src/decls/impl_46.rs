macro_rules! deps {
    () => {
        Ones!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a > Iterator for Ones < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { while self . bitset_front == 0 { match self . remaining_blocks . next () { Some (next_block) => { self . bitset_front = * next_block ; self . block_idx_front += BITS ; } None => { if self . bitset_back != 0 { self . block_idx_front = self . block_idx_back ; self . bitset_front = 0 ; return Some (self . block_idx_back + Self :: last_positive_bit_and_unset (& mut self . bitset_back) ,) ; } else { return None ; } } } ; } Some (self . block_idx_front + Self :: last_positive_bit_and_unset (& mut self . bitset_front)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , (Some (self . block_idx_back - self . block_idx_front + 2 * BITS)) ,) } }
    };
}

impl_46!()