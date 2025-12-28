macro_rules! deps {
    () => {
        TaggedArcPtr!();
    };
}

macro_rules! increase_arc_refcount {
    () => {
        deps!();
        fn increase_arc_refcount (repr : TaggedArcPtr) -> TaggedArcPtr { let Some (arc) = (unsafe { repr . try_as_arc_owned () }) else { return repr ; } ; mem :: forget (Arc :: clone (& arc)) ; repr }
    };
}

increase_arc_refcount!();