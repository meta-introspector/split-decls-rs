macro_rules! deps {
    () => {
        ExecWaker!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl futures_util :: task :: ArcWake for ExecWaker { fn wake_by_ref (me : & Arc < ExecWaker >) { me . 0 . store (true , Ordering :: SeqCst) ; } }
    };
}

impl_346!();