macro_rules! deps {
    () => {
        InputObject!();
        Interface!();
        Union!();
        Object!();
        Scalar!();
    };
}

macro_rules! MetaTypeId {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum MetaTypeId { Scalar , Object , Interface , Union , Enum , InputObject , }
    };
}

MetaTypeId!()