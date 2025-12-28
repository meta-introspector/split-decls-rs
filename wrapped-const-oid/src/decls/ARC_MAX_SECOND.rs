macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! ARC_MAX_SECOND {
    () => {
        deps!();
        # [doc = " Maximum value of the second arc in an OID."] pub (crate) const ARC_MAX_SECOND : Arc = 39 ;
    };
}

ARC_MAX_SECOND!();