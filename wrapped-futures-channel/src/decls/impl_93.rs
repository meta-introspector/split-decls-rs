macro_rules! deps {
    () => {
        BoundedInner!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T > BoundedInner < T > { fn max_senders (& self) -> usize { MAX_CAPACITY - self . buffer } fn set_closed (& self) { let curr = self . state . load (SeqCst) ; if ! decode_state (curr) . is_open { return ; } self . state . fetch_and (! OPEN_MASK , SeqCst) ; } }
    };
}

impl_93!();