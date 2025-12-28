macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T > Drop for Channel < T > { fn drop (& mut self) { if mem :: needs_drop :: < T > () { let head = * self . head . get_mut () ; let tail = * self . tail . get_mut () ; let hix = head & (self . mark_bit - 1) ; let tix = tail & (self . mark_bit - 1) ; let len = if hix < tix { tix - hix } else if hix > tix { self . cap () - hix + tix } else if (tail & ! self . mark_bit) == head { 0 } else { self . cap () } ; for i in 0 .. len { let index = if hix + i < self . cap () { hix + i } else { hix + i - self . cap () } ; unsafe { debug_assert ! (index < self . buffer . len ()) ; let slot = self . buffer . get_unchecked_mut (index) ; (* slot . msg . get ()) . assume_init_drop () ; } } } } }
    };
}

impl_104!()