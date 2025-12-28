macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T > Drop for Channel < T > { fn drop (& mut self) { let mut head = * self . head . index . get_mut () ; let mut tail = * self . tail . index . get_mut () ; let mut block = * self . head . block . get_mut () ; head &= ! ((1 << SHIFT) - 1) ; tail &= ! ((1 << SHIFT) - 1) ; unsafe { while head != tail { let offset = (head >> SHIFT) % LAP ; if offset < BLOCK_CAP { let slot = (* block) . slots . get_unchecked (offset) ; (* slot . msg . get ()) . assume_init_drop () ; } else { let next = * (* block) . next . get_mut () ; drop (Box :: from_raw (block)) ; block = next ; } head = head . wrapping_add (1 << SHIFT) ; } if ! block . is_null () { drop (Box :: from_raw (block)) ; } } } }
    };
}

impl_131!()