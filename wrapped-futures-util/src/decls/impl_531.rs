macro_rules! deps {
    () => {
        WrappedWaker!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl ArcWake for WrappedWaker { fn wake_by_ref (self_arc : & Arc < Self >) { if let Some ((_ , state_bomb)) = self_arc . start_waking () { let waker_opt = unsafe { self_arc . inner_waker . get () . as_ref () . unwrap () } ; if let Some (inner_waker) = waker_opt . clone () { drop (state_bomb) ; inner_waker . wake () ; } } } }
    };
}

impl_531!();