macro_rules! deps {
    () => {
        WakerInner!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl ArcWake for WakerInner { fn wake_by_ref (arc_self : & Arc < Self >) { let _ = arc_self . count . fetch_add (1 , Ordering :: SeqCst) ; } }
    };
}

impl_39!();