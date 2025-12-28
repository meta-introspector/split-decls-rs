macro_rules! deps {
    () => {
        Notifier!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl ArcWake for Notifier { fn wake_by_ref (arc_self : & Arc < Self >) { # [cfg (feature = "std")] let wakers = & mut * arc_self . wakers . lock () . unwrap () ; # [cfg (not (feature = "std"))] let wakers = & mut * arc_self . wakers . lock () ; if let Some (wakers) = wakers . as_mut () { for (_key , opt_waker) in wakers { if let Some (waker) = opt_waker . take () { waker . wake () ; } } } } }
    };
}

impl_105!();