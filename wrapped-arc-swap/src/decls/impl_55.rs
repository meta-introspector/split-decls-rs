macro_rules! deps {
    () => {
        Local!();
        Debt!();
        Slots!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Slots { # [doc = " Try to allocate one slot and get the pointer in it."] # [doc = ""] # [doc = " Fails if there are no free slots."] # [inline] pub (super) fn get_debt (& self , ptr : usize , local : & Local) -> Option < & Debt > { let offset = local . offset . get () ; let len = self . 0 . len () ; for i in 0 .. len { let i = (i + offset) % len ; let slot = & self . 0 [i] ; if slot . 0 . load (Relaxed) == Debt :: NONE { let old = slot . 0 . swap (ptr , SeqCst) ; debug_assert_eq ! (Debt :: NONE , old) ; local . offset . set (i + 1) ; return Some (& self . 0 [i]) ; } } None } }
    };
}

impl_55!()