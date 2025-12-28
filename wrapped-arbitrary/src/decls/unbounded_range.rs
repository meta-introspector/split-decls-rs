macro_rules! unbounded_range {
    () => {
        pub (crate) fn unbounded_range < CB , I , R > (bound : I , cb : CB) -> R where CB : Fn (I) -> R , R : RangeBounds < I > , { cb (bound) }
    };
}

unbounded_range!();