macro_rules! deps {
    () => {
        ThreadNotify!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl ArcWake for ThreadNotify { fn wake_by_ref (arc_self : & Arc < Self >) { let unparked = arc_self . unparked . swap (true , Ordering :: Release) ; if ! unparked { arc_self . thread . unpark () ; } } }
    };
}

impl_6!();