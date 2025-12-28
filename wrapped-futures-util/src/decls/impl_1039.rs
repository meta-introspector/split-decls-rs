macro_rules! deps {
    () => {
        Current!();
    };
}

macro_rules! impl_1039 {
    () => {
        deps!();
        impl ArcWake03 for Current { fn wake_by_ref (arc_self : & Arc < Self >) { arc_self . 0 . notify () ; } }
    };
}

impl_1039!();