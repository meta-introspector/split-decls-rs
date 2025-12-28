macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Iterator for Masks { type Item = (usize , usize) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . first_block . cmp (& self . last_block) { Ordering :: Less => { let res = (self . first_block , self . first_mask) ; self . first_block += 1 ; self . first_mask = ! 0 ; Some (res) } Ordering :: Equal => { let mask = self . first_mask & self . last_mask ; let res = if mask == 0 { None } else { Some ((self . first_block , mask)) } ; self . first_block += 1 ; res } Ordering :: Greater => None , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . first_block ..= self . last_block) . size_hint () } }
    };
}

impl_109!();