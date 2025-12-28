macro_rules! deps {
    () => {
        UnboundedInner!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > UnboundedInner < T > { fn set_closed (& self) { let curr = self . state . load (SeqCst) ; if ! decode_state (curr) . is_open { return ; } self . state . fetch_and (! OPEN_MASK , SeqCst) ; } }
    };
}

impl_92!()