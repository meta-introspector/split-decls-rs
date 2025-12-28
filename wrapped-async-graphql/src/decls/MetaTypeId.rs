macro_rules! deps {
    () => {
        Interface!();
        Union!();
        Scalar!();
        InputObject!();
        Object!();
    };
}

macro_rules! MetaTypeId {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum MetaTypeId { Scalar , Object , Interface , Union , Enum , InputObject , }
    };
}

MetaTypeId!();