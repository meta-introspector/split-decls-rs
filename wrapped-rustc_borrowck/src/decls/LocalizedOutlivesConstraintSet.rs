macro_rules! deps {
    () => {
        LocalizedOutlivesConstraint!();
    };
}

macro_rules! LocalizedOutlivesConstraintSet {
    () => {
        deps!();
        # [doc = " A container of [LocalizedOutlivesConstraint]s that can be turned into a traversable"] # [doc = " `rustc_data_structures` graph."] # [derive (Clone , Default , Debug)] pub (crate) struct LocalizedOutlivesConstraintSet { pub outlives : Vec < LocalizedOutlivesConstraint > , }
    };
}

LocalizedOutlivesConstraintSet!()