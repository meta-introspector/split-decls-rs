macro_rules! Compatibility {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) enum Compatibility < 'tcx > { Compatible , Incompatible (Option < TypeError < 'tcx > >) , }
    };
}

Compatibility!();