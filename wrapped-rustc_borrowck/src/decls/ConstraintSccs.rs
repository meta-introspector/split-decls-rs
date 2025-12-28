macro_rules! ConstraintSccs {
    () => {
        pub (crate) type ConstraintSccs = Sccs < RegionVid , ConstraintSccIndex > ;
    };
}

ConstraintSccs!();