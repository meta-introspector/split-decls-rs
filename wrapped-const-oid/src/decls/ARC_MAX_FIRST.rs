macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! ARC_MAX_FIRST {
    () => {
        deps!();
        # [doc = " Maximum value of the first arc in an OID."] pub (crate) const ARC_MAX_FIRST : Arc = 2 ;
    };
}

ARC_MAX_FIRST!()